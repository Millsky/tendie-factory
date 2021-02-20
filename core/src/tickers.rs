use std::collections::{ HashSet };

const NASDAQ_TICKER_PATH: &str = "./src/nasdaq_screener.csv";

pub fn get_tickers_nasdaq() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(NASDAQ_TICKER_PATH)?;
    let tickers: HashSet<String> = rdr.records()
        .map(| r | { String::from(&r.unwrap()[0]) } )
        .collect();
    Ok(tickers)
}
