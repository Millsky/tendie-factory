use std::collections::{HashSet, HashMap};
use dashmap::{DashMap};
use rayon::prelude::*;
use std::collections::hash_map::RandomState;
use rust_bert::{
    pipelines::{
        sentiment::{SentimentPolarity},
    }
};
use serde::{Deserialize, Serialize};

mod nlp;

#[derive(Debug, Clone)]
struct Title {
    title: String,
    tickers: Vec<String>,
    sentiment: f64,
}

impl Title {
    fn parse_for_tickers(self, available_tickers: &HashSet<String>) -> Self {
        let mut possible_tickers = nlp::bert_organization_tokenization(String::from(&self.title));
        let mut actual_tickers: Vec<String> = vec![];
        println!("{}", self.title);
        self.title.split(" ").map(String::from).for_each(|token| {
            if actual_tickers.contains(&token) {
                possible_tickers.push(token);
            }
        });

        for ticker in possible_tickers {
            if available_tickers.contains(&ticker) {
                actual_tickers.push(String::from(ticker));
            }
        }
        Title {  tickers: actual_tickers, ..self }
    }

    fn get_sentiment(self) -> Self {
        let sentiment = &nlp::sentiment_classifier(&self.title)[0];
        let title_sentiment = match sentiment {
            SentimentPolarity::Positive => {
                1.0
            },
            SentimentPolarity::Negative => {
                -1.0
            }
        };
        Title { sentiment: title_sentiment, ..self }
    }

    fn has_tickers(&self) -> bool {
        return self.tickers.len() > 0;
    }

    fn new(title: &str) -> Self {
        let t = title.chars()
            .filter(|c| c.is_ascii())
            .filter(|c | !c.is_ascii_punctuation())
            .collect::<String>();
        Title {
            title: t,
            tickers: vec![],
            sentiment: 0.0,
        }
    }
}

#[test]
fn can_create_title() {
    let m: Title = Title::new("My Post title here");
    assert_eq!(m.title, "My Post title here");
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerData {
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

impl Default for TickerData {
    fn default() -> Self {
        TickerData {
            symbol: "".to_string(),
            company_name: "".to_string(),
            last_sale: "".to_string(),
            net_change: "".to_string(),
            net_percent_change: "".to_string(),
            market_capitalization: "".to_string(),
            country: "".to_string(),
            ipo_year: "".to_string(),
            volume: "".to_string(),
            sector: "".to_string(),
            industry: "".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    occurrences: i32,
    ticker: String,
    titles_with_ticker: Vec<String>,
    sentiment: f64,
    portfolio_weight: f64,
    meta: TickerData,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics {
            occurrences: 0,
            ticker: "".parse().unwrap(),
            titles_with_ticker: vec![],
            sentiment: 0.0,
            portfolio_weight: 0.0,
            meta: TickerData::default(),
        }
    }
}

const NASDAQ_TICKER_PATH: &str = "./src/nasdaq_screener.csv";

pub fn get_ticker_meta_data() -> Result<HashMap<String, TickerData>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(NASDAQ_TICKER_PATH)?;
    let mut ticker_data: HashMap<String, TickerData> = HashMap::new();
    for result in rdr.deserialize() {
        let record: TickerData = result?;
        ticker_data.insert(String::from(&record.symbol), record);
    }
    Ok(ticker_data)
}

pub fn derive_metrics(available_tickers: HashSet<String>, titles: Vec<String>) -> DashMap<String, Metrics, RandomState> {
    let titles_with_tickers: Vec<Title> = titles
        .par_iter()
        .map(| title | {
            Title::new(title)
        })
        .map(|title: Title | {
            title.parse_for_tickers(&available_tickers)
        })
        .filter(| title | {
            title.has_tickers()
        })
        .map(| title: Title | {
            title.get_sentiment()
        })
        .collect();

    let ticker_meta_data = get_ticker_meta_data().unwrap();

    println!("{:?}", titles_with_tickers);

    let stock_metrics: DashMap<String, Metrics> = DashMap::new();

    titles_with_tickers.par_iter().for_each(|title: &Title | {
        title.tickers.par_iter().for_each(| ticker | {
            if !stock_metrics.contains_key(ticker) {
                let td = ticker_meta_data.get(ticker).unwrap();
                stock_metrics.insert(String::from(ticker), Metrics {
                    occurrences: 1,
                    ticker: ticker.clone().to_string(),
                    titles_with_ticker: vec![title.title.clone()],
                    sentiment: title.sentiment,
                    portfolio_weight: 1.0 /  titles_with_tickers.len() as f64,
                    meta: td.clone(),
                });
            } else {
                let mut metrics = stock_metrics.get_mut(ticker).unwrap();
                metrics.occurrences+=1;
                metrics.titles_with_ticker.push(title.title.clone());
                metrics.sentiment = (metrics.sentiment + title.sentiment) / metrics.occurrences as f64;
                metrics.portfolio_weight = metrics.occurrences as f64 / titles_with_tickers.len() as f64;
            }
        });
    });

    stock_metrics
}