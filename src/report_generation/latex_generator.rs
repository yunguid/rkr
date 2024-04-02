use serde_json::Value;
use std::fs::File;
use std::io::{Write, Error};
use std::process::Command;
extern crate serde_json;
extern crate serde;


pub fn extract_completion_from_json(json_data: &str) -> Result<String, serde_json::Error> {
    let parsed: Value = serde_json::from_str(json_data)?;
    Ok(parsed["completion"].as_str().unwrap().to_string())
}

/// Parses the JSON string to extract the summary data.
pub fn parse_summary_data(json_data: &str) -> Result<Value, serde_json::Error> {
    let parsed: Value = serde_json::from_str(json_data)?;
    Ok(parsed["completion"].clone())
}

/// Creates a LaTeX document string from the summary data.
pub fn create_latex_document(summary: &str, symbol: &str) -> String {
    format!(
        r#"\documentclass{{article}}
\usepackage{{hyperref}}
\begin{{document}}

\title{{Summary Report for {}}}
\date{{\today}}
\maketitle

{}

\end{{document}}
"#,
        symbol, summary.replace("•", "\\item ")
    )
}

/// Generates a `.tex` file from the LaTeX document string.
pub fn generate_latex_file(latex_content: &str, file_path: &str) -> Result<(), Error> {
    let mut file = File::create(file_path)?;
    file.write_all(latex_content.as_bytes())?;
    Ok(())
}

/// Compiles the LaTeX file to a PDF using the `pdflatex` command.
pub fn compile_latex_to_pdf(file_path: &str) -> Result<(), Error> {
    Command::new("pdflatex")
        .arg(file_path)
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_summary_data() {
        let json_data = r#"{"completion": "Here is the summary..."}"#;
        let parsed_data = parse_summary_data(json_data).unwrap();
        assert!(parsed_data.is_string());
        assert_eq!(parsed_data, "Here is the summary...");
    }

    #[test]
    fn test_latex_document_creation() {
        let summary = "Here is the summary...";
        let symbol = "NVDA";
        let latex_doc = create_latex_document(summary, symbol);
        assert!(latex_doc.contains("\\title{Summary Report for NVDA}"));
        assert!(latex_doc.contains(summary));
    }

    // Additional tests for file generation and LaTeX compilation could be added here
}
