use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use rust_bert::pipelines::sentiment::SentimentPolarity;
use std::collections::hash_map::RandomState;

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

#[derive(Debug, Deserialize, Serialize)]
struct TickerMetrics {
    occurrences: i32,
    ticker: String,
    titles_with_ticker: Vec<String>,
    sentiment: f64,
    portfolio_weight: f64,
}

impl TickerMetrics {
    fn new(title: String, ticker: String) -> Self {
        TickerMetrics {
            occurrences: 1,
            ticker,
            titles_with_ticker: vec![title],
            sentiment: 0.0,
            portfolio_weight: 0.0,
        }
    }

    fn increment_occurrences(&mut self) {
        self.occurrences += 1;
    }

    fn add_title(&mut self, title: String) {
        self.titles_with_ticker.push(title);
    }

    fn set_portfolio_weight(&mut self, weight: f64) {
        self.portfolio_weight = weight;
    }

    fn calculate_sentiment(&mut self) {
        let mut sentiment = 0.0;
        let sentiments = nlp::sentiment_classifier(&self.titles_with_ticker);
        for s  in &sentiments {
            match s {
                SentimentPolarity::Negative => {
                    sentiment -= 1.0;
                }
                SentimentPolarity::Positive => {
                    sentiment += 1.0;
                }
            }
        }
        self.sentiment = (sentiment / sentiments.len() as f64);
    }
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
fn get_metrics_for_tickers(posts: Vec<RedditContainer<RedditPost>>, tickers: Vec<String>) -> HashMap<String, TickerMetrics> {
    let mut ticker_metrics: HashMap<String, TickerMetrics> = HashMap::new();

    let titles: Vec<String> = posts.iter().map(|p| String::from(p.data.title.clone())).collect();
    let possible_companies_list = nlp::bert_organization_tokenization(&titles);

    for (i, post) in posts.iter().enumerate() {
        let possible_companies = &possible_companies_list[i];
        for token in post.data.title.split(" ").map(String::from) {
            let first_char = token.chars().nth(0).unwrap();
            let mut current_ticker: String = "".to_string();
            if tickers.contains(&token) && possible_companies.contains(&token) {
                current_ticker = String::from(&token);
            } else if first_char == '$' {
                let cash_tagged_ticker: Vec<String> = token.split("$").map(|s| String::from(s)).collect();
                if tickers.contains(&cash_tagged_ticker[1]) {
                    current_ticker = String::from(&cash_tagged_ticker[1]);
                }
            }
            if ticker_metrics.contains_key(current_ticker.as_str()) {
                let ticker_metric = ticker_metrics.get_mut(current_ticker.as_str()).unwrap();
                ticker_metric.increment_occurrences();
                ticker_metric.add_title(String::from(&post.data.title));
            } else if current_ticker != "" {
                let ticker_metric = TickerMetrics::new(String::from(&post.data.title), String::from(&current_ticker));
                ticker_metrics.insert(current_ticker, ticker_metric);
            }
        }
    }

   for tm in ticker_metrics.values_mut() {
       tm.calculate_sentiment();
   }

    ticker_metrics
}

// Calculates an optimal portfolio weight using the naive metric of (mentions of ticker / total posts that mention a ticker)
fn calculate_portfolio_weights_simple(ticker_metrics: &mut HashMap<String, TickerMetrics>) -> &mut HashMap<String, TickerMetrics, RandomState> {
    // For now we are assuming the sentiment of each ticker is positive, since STONKS ONLY GO UP
    let total_mentions = ticker_metrics.values().fold(0, |acc, w| { acc + w.occurrences });
    for metric in ticker_metrics.values_mut() {
        metric.set_portfolio_weight(metric.occurrences as f64 / total_mentions as f64);
    }
    ticker_metrics
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
    let mut ticker_metrics = get_metrics_for_tickers(posts.data.children, tickers);
    // 3. Determine the weight of each of the posts talking about a given ticker
    let portfolio_weights = calculate_portfolio_weights_simple(&mut ticker_metrics);
    println!("{:?}", portfolio_weights);
    // 4. Construct a portfolio of stocks based on this initial weighting
    // let portfolio = create_portfolio(portfolio_weights, &ticker_metrics);
    // let json_portfolio = serde_json::to_string(&portfolio)?;
    // println!("{:?}", json_portfolio);
    // let mut file = File::create("portfolio.json")?;
    // file.write_all(json_portfolio.as_bytes())?;
    Ok(())
}
