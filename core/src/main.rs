use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod nlp;

#[derive(Serialize, Deserialize)]
struct RedditContainer<T> {
    kind: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct RedditPost {
    title: String,
}

#[derive(Serialize, Deserialize)]
struct RedditListing {
    children: Vec<RedditContainer<RedditPost>>
}

#[derive(Debug, Deserialize, Serialize)]
struct TickerData {
    #[serde(rename(deserialize = "Symbol"))]
    symbol: String,
    #[serde(rename(deserialize = "Name"))]
    company_name: String,
    #[serde(rename(deserialize = "Last Sale"))]
    last_sale: String,
    #[serde(rename(deserialize = "Net Change"))]
    net_change: String,
    #[serde(rename(deserialize = "% Change"))]
    net_percent_change: String,
    #[serde(rename(deserialize = "Market Cap"))]
    market_capitalization: String,
    #[serde(rename(deserialize = "Country"))]
    country: String,
    #[serde(rename(deserialize = "IPO Year"))]
    ipo_year: String,
    #[serde(rename(deserialize = "Volume"))]
    volume: String,
    #[serde(rename(deserialize = "Sector"))]
    sector: String,
    #[serde(rename(deserialize = "Industry"))]
    industry: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PortfolioItem {
    meta: TickerData,
    portfolio_weight: f64,
}

fn get_tickers_nasdaq() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path("./src/nasdaq_screener.csv")?;
    let mut tickers: Vec<String> = vec![];
    for result in rdr.records() {
        let record = result?;
        tickers.push(String::from(&record[0]));
    }
    Ok(tickers)
}

fn pull_ticker_meta_data(tickers: Vec<String>) -> Result<HashMap<String, TickerData>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path("./src/nasdaq_screener.csv")?;
    let mut ticker_data: HashMap<String, TickerData> = HashMap::new();
    for result in rdr.deserialize() {
        let record: TickerData = result?;
        if tickers.contains(&record.symbol) {
            ticker_data.insert(String::from(&record.symbol), record);
        }
    }
    Ok(ticker_data)
}

async fn get_wsb_top() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.reddit.com/r/wallstreetbets/new/.json?limit=100&t=month")
    .await?
    .text()
    .await?;
    Ok(body)
}

// Calculates the number of mentions for each ticker
fn get_metrics_for_tickers(posts: Vec<RedditContainer<RedditPost>>, tickers: Vec<String>) -> HashMap<String, i32> {
    let mut tickers_in_each_title: Vec<HashSet<String>> = vec![];
    for post in posts.into_iter() {
        let mut tickers_in_title: HashSet<String> = HashSet::new();
        let possible_companies = nlp::bert_organization_tokenization(post.data.title.as_str());
        for token in post.data.title.split(" ").map(String::from) {
            let first_char = token.chars().nth(0).unwrap();
            if tickers.contains(&token) && possible_companies.contains(&token) {
                tickers_in_title.insert(String::from(&token));
            }
            // Handle Cash Tagged Assets ex: $GME
            if first_char == '$' {
                let cash_tagged_ticker: Vec<String> = token.split("$").map(|s| String::from(s)).collect();
                if tickers.contains(&cash_tagged_ticker[1]) {
                    tickers_in_title.insert(String::from(&cash_tagged_ticker[1]));
                }
            }
        }
        tickers_in_each_title.push(tickers_in_title);
    }

    let mut ticker_metrics: HashMap<String, i32> = HashMap::new();
    for ticker_matches in tickers_in_each_title {
        for ticker in ticker_matches.into_iter() {
            match ticker_metrics.contains_key(&ticker) {
                true => {
                    ticker_metrics.insert(String::from(&ticker), *ticker_metrics.get(&ticker).unwrap() + 1);
                },
                false =>  {
                    ticker_metrics.insert(String::from(&ticker), 1);
                }
            }
        }
    }
    ticker_metrics
}

// Calculates an optimal portfolio weight using the naive metric of (mentions of ticker / total posts that mention a ticker)
fn calculate_portfolio_weights_simple(ticker_metrics: HashMap<String, i32>) -> HashMap<String, f64> {
    // For now we are assuming the sentiment of each ticker is positive, since STONKS ONLY GO UP
    let mut portfolio_weights: HashMap<String, f64> = HashMap::new();
    let total_mentions = ticker_metrics.values().into_iter().fold(0, |acc, w| { acc + w});
    for (k, v) in ticker_metrics.into_iter() {
        portfolio_weights.insert(k, v as f64 / total_mentions as f64);
    }
    portfolio_weights
}

fn create_portfolio(ticker_metrics: HashMap<String, f64>) -> Vec<PortfolioItem> {
    let ticker_meta_data = pull_ticker_meta_data( ticker_metrics.keys().map(String::from).collect()).unwrap();
    ticker_meta_data.into_iter().map(| (key, meta) | {
        let pi = PortfolioItem {
            meta,
            portfolio_weight: ticker_metrics.get(key.as_str()).unwrap().clone(),
        };
        return pi;
    }).collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Pull all the stock tickers and convert to a vec
    let tickers = get_tickers_nasdaq()?;
    // 2. Download and parse the reddit WSB data
    let posts: RedditContainer<RedditListing> = serde_json::from_str(
        &get_wsb_top().await?
    )?;
    // 3. Compute the number of occurrences of each ticker in each title
    let ticker_metrics = get_metrics_for_tickers(posts.data.children, tickers);
    // 3. Determine the weight of each of the posts talking about a given ticker
    let portfolio_weights = calculate_portfolio_weights_simple(ticker_metrics);
    // 4. Construct a portfolio of stocks based on this initial weighting
    let portfolio = create_portfolio(portfolio_weights);
    let json_portfolio = serde_json::to_string(&portfolio)?;
    let mut file = File::create("portfolio.json")?;
    file.write_all(json_portfolio.as_bytes())?;
    Ok(())
}
