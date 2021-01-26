use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::collections::HashMap;

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

fn get_tickers_nasdaq() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path("./src/companylist.csv")?;
    let mut tickers: Vec<String> = vec![];
    for result in rdr.records() {
        let record = result?;
        tickers.push(String::from(&record[0]));
    }
    tickers.push(String::from("GME"));
    tickers.push(String::from("BB"));
    tickers.push(String::from("PLTR"));
    Ok(tickers)
}

async fn get_wsb_top() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.reddit.com/r/wallstreetbets/top/.json?count=25&t=week")
    .await?
    .text()
    .await?;
    Ok(body)
}

fn get_metrics_for_tickers(posts: Vec<RedditContainer<RedditPost>>, tickers: Vec<String>) -> HashMap<String, i32> {
    let tickers_in_each_title: Vec<HashSet<&String>> = posts.into_iter().map(|x| {
        x.data.title
    }).map(| x | {
        let mut tickers_in_title = HashSet::new();
        for ticker in &tickers {
            // Found at start of line
            match twoway::find_str(&x, &format!("{} ", ticker)) {
                Some(_n) => {
                    tickers_in_title.insert(ticker);
                },
                None => {}
            }
            // Found at end of line
            match twoway::find_str(&x, &format!(" {}", ticker)) {
                Some(_n) => {
                    tickers_in_title.insert(ticker);
                },
                None => {}
            }
            // Found in middle of line
            match twoway::find_str(&x, &format!(" {} ", ticker)) {
                Some(_n) => {
                    tickers_in_title.insert(ticker);
                },
                None => {}
            }
            // Found in ticker
            match twoway::find_str(&x, &format!("${}", ticker)) {
                Some(_n) => {
                    tickers_in_title.insert(ticker);
                },
                None => {}
            }
        }
        return tickers_in_title;
    }).collect();

    let mut ticker_metrics: HashMap<String, i32> = HashMap::new();
    for ticker_matches in tickers_in_each_title {
        for ticker in ticker_matches.into_iter() {
            match ticker_metrics.contains_key(ticker) {
                true => {
                    ticker_metrics.insert(String::from(ticker), *ticker_metrics.get(ticker).unwrap() + 1);
                },
                false =>  {
                    ticker_metrics.insert(String::from(ticker), 1);
                }
            }
        }
    }
    return ticker_metrics;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Pull all the stock tickers and convert to a vec
    let tickers = get_tickers_nasdaq()?;
    // 2. Download and parse the reddit WSB data
    let posts: RedditContainer<RedditListing> = serde_json::from_str(
        &get_wsb_top().await?
    )?;
    // 3. Calculate the number of occurrences of each ticker in each title
    let ticker_metrics = get_metrics_for_tickers(posts.data.children, tickers);
    println!("{:?}", ticker_metrics);
    // 3. TODO: Determine the weight of each of the posts talking about a given ticker
    // 4. TODO: Construct a portfolio of stocks based on this initial weighting
    Ok(())
}
