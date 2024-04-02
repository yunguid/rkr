use std::io::{self, Write};
use std::fs::{OpenOptions};
use std::path::Path;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};
use env_logger;
use chrono::Utc;
use crate::data_ingestion::data_provider::AggregateResponse;
use crate::report_generation::latex_generator;


mod data_ingestion;
mod claude_api;
mod report_generation {
    pub mod latex_generator;
}

const PORTFOLIO_FILE: &str = "portfolios.txt";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting RKR trading system...");

    // Load existing portfolios
    let portfolios = load_portfolios();

    println!("Press '1' to review existing portfolios or '2' to create a new one:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let symbols = if input == "1" {
        // Display existing portfolios and prompt for selection
        println!("Existing portfolios:");
        for (i, portfolio) in portfolios.iter().enumerate() {
            println!("{}: {:?}", i + 1, portfolio);
        }
        println!("Enter the portfolio number you would like to select:");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input");
        let index = selection.trim().parse::<usize>().expect("Invalid portfolio number");
        portfolios[index - 1].clone()
    } else if input == "2" {
        // Create a new portfolio
        println!("Enter the stock symbols for your portfolio (press Enter after each symbol, and press Enter again when done):");
        let mut symbols = Vec::new();
        loop {
            let mut symbol = String::new();
            io::stdin().read_line(&mut symbol).expect("Failed to read input");
            let symbol = symbol.trim();
            if symbol.is_empty() {
                break;
            }
            symbols.push(symbol.to_string());
        }
        save_portfolio(&symbols);
        println!("Portfolio saved successfully!");
        symbols
    } else {
        println!("Invalid input. Exiting...");
        return;
    };

    // Set the start and end dates for the past 6 months
    let end_date = Utc::now().format("%Y-%m-%d").to_string();
    let start_date = (Utc::now() - chrono::Days::new(180)).format("%Y-%m-%d").to_string();

    let symbols_clone = symbols.clone();
    let start_date_clone = start_date.clone();
    let end_date_clone = end_date.clone();

    let market_data = match data_ingestion::fetch_market_data(&symbols_clone, &start_date_clone, &end_date_clone).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch market data: {}", e);
            Vec::new()
        }
    };

    let market_data_arc = Arc::new(Mutex::new(market_data));
    let _market_data_clone = Arc::clone(&market_data_arc);

    info!("Generating summary report...");
    generate_summary_report(&symbols, &start_date, &end_date, market_data_arc).await;

    info!("Shutting down the RKR trading system...");
}

fn load_portfolios() -> Vec<Vec<String>> {
    if Path::new(PORTFOLIO_FILE).exists() {
        let contents = fs::read_to_string(PORTFOLIO_FILE).expect("Failed to read portfolio file");
        contents.lines()
            .map(|line| line.split_whitespace().map(|s| s.trim().to_string()).collect())
            .collect()
    } else {
        Vec::new()
    }
}


fn save_portfolio(symbols: &[String]) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(PORTFOLIO_FILE)
        .expect("Failed to open portfolio file");

    writeln!(file, "{}", symbols.join(",")).expect("Failed to write to portfolio file");
}

async fn generate_summary_report(symbols: &[String], start_date: &str, end_date: &str, market_data_arc: Arc<Mutex<Vec<AggregateResponse>>>) {
    let market_data = market_data_arc.lock().await;
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    fs::create_dir_all(format!("reports/{}", current_date)).expect("Failed to create reports directory");

    for symbol in symbols {
        // Filter market data for the current symbol
        let symbol_data: Vec<&AggregateResponse> = market_data
            .iter()
            .filter(|data| data.ticker == *symbol)
            .collect();

        if let Some(data) = symbol_data.first() {
            // Extract key metrics and statistics
            let opening_price = data.results.first().map(|result| result.o).unwrap_or(0.0);
            let closing_price = data.results.last().map(|result| result.c).unwrap_or(0.0);
            let highest_price = data.results.iter().map(|result| result.h).fold(f64::MIN, f64::max);
            let lowest_price = data.results.iter().map(|result| result.l).fold(f64::MAX, f64::min);
            let average_volume = data.results.iter().map(|result| result.v).sum::<f64>() / data.results.len() as f64;
            let percentage_change = (closing_price - opening_price) / opening_price * 100.0;

            // Fetch additional information from the web (example using dummy data)
            let company_info = format!("Company: {}\nIndustry: Technology\nHeadquarters: City, Country", symbol);

            // Prepare the summary for the current symbol
            let symbol_summary = format!(
                "Symbol: {}\nDate Range: {} to {}\n\nKey Metrics:\nOpening Price: ${:.2}\nClosing Price: ${:.2}\nHighest Price: ${:.2}\nLowest Price: ${:.2}\nAverage Volume: {:.2}\nPercentage Change: {:.2}%\n\nCompany Information:\n{}",
                symbol,
                start_date,
                end_date,
                opening_price,
                closing_price,
                highest_price,
                lowest_price,
                average_volume,
                percentage_change,
                company_info
            );

            // Generate the summary using the Claude API
            match claude_api::generate_summary(&symbol_summary).await {
                Ok(summary) => {
                    // Use the latex_generator module to convert summary into LaTeX format

                    let latex_content = report_generation::latex_generator::create_latex_document(&summary, &symbol);
                    let latex_file_path = format!("reports/{}/{}_summary.tex", current_date, symbol);
                    let pdf_file_path = format!("reports/{}/{}_summary.pdf", current_date, symbol);
                    
                    // Generate LaTeX file
                    if let Err(e) = report_generation::latex_generator::generate_latex_file(&latex_content, &latex_file_path) {
                        error!("Failed to generate LaTeX file for {}: {}", symbol, e);
                        continue;
                    }
                    
                    // Compile LaTeX file to PDF
                    if let Err(e) = report_generation::latex_generator::compile_latex_to_pdf(&latex_file_path) {
                        error!("Failed to compile LaTeX file to PDF for {}: {}", symbol, e);
                        continue;
                    }
                    
                    info!("Generated LaTeX PDF report for {}: {}", symbol, pdf_file_path);
                }
                Err(e) => {
                    error!("Failed to generate summary report for {}: {}", symbol, e);
                }
            }
        }
    }
}