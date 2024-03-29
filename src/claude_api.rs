use reqwest::Client;
use serde::Serialize;

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/complete";
const CLAUDE_API_KEY: &str = "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";

#[derive(Serialize)]
struct ClaudeRequest {
    prompt: String,
    model: String,
    max_tokens_to_sample: usize,
}

pub async fn generate_summary(portfolio_data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request_body = ClaudeRequest {
        prompt: format!("
Human:

Please provide a detailed and informative summary of the following portfolio data:

{}

Include the following sections in your summary:

1. Overview: Provide a brief overview of the stock's performance during the given date range.

2. Key Metrics:
   - Highlight the opening and closing prices, highest and lowest prices, average volume, and percentage change.
   - Provide a brief interpretation of these metrics and what they indicate about the stock's performance.

3. Company Information:
   - Include relevant information about the company, such as its industry, products/services, and any recent news or developments.

4. Analysis and Insights:
   - Analyze the stock's performance and provide insights into potential factors influencing its price movements.
   - Discuss any notable trends, patterns, or events that may have impacted the stock during the given period.

5. Conclusion:
   - Summarize the overall performance of the stock and provide a concise conclusion based on the analysis.

Please format the summary in a clear and readable manner, using appropriate headings, bullet points, and paragraphs.

Assistant:

", portfolio_data),
        model: "claude-v1".to_string(),
        max_tokens_to_sample: 500,
    };

    let response = client
        .post(CLAUDE_API_URL)
        .header("Content-Type", "application/json")
        .header("x-api-key", CLAUDE_API_KEY)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}