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
    let tickers_in_each_title = posts.into_iter().map(|reddit_post| {
        reddit_post.data.title.split(" ").map(|s| String::from(s)).collect()
    }).map(| reddit_post_title_tokens: Vec<String> | {
        reddit_post_title_tokens.into_iter().fold(HashSet::new(), |mut acc, title_token| {
            if tickers.contains(&title_token) {
                acc.insert(String::from(&title_token));
            }
            // handle Cash Tagged Assets: EX: $GME
            let first_char = title_token.chars().nth(0).unwrap();
            match first_char {
                '$' => {
                    let cash_tagged_ticker: Vec<String> = title_token.split("$").map(|s| String::from(s)).collect();
                    if tickers.contains(&String::from(&cash_tagged_ticker[1])) {
                        acc.insert((*String::from(&cash_tagged_ticker[1])).parse().unwrap());
                    }
                },
                _ => {},
            }
            acc
        })
    });

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
