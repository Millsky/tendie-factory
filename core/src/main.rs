use std::fs::File;
use std::io::Write;
use dashmap::{DashMap};

mod metrics;
mod reddit;
mod tickers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let titles = reddit::get_post_titles().await?;
    let tickers = tickers::get_tickers_nasdaq().unwrap();
    let metrics = DashMap::into_read_only(metrics::derive_metrics(tickers, titles));
    let metrics_formatted: Vec<&metrics::Metrics> = metrics.values().collect();
    let json_portfolio = serde_json::to_string(&metrics_formatted)?;
    println!("{:?}", json_portfolio);
    let mut file = File::create("portfolio.json")?;
    file.write_all(json_portfolio.as_bytes())?;
    Ok(())
}
