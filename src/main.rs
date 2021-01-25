use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    Ok(tickers)
}

async fn get_wsb_top() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.reddit.com/r/wallstreetbets/top/.json?count=20")
    .await?
    .text()
    .await?;
    Ok(body)
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
    let tickers_in_each_title: Vec<HashSet<&String>> = posts.data.children.into_iter().map(|x| {
        x.data.title
    }).map(| x | {
        let mut tickers_in_title = HashSet::new();
        println!("{}", x);
        for ticker in &tickers {
            match twoway::find_str(&x, &format!(" {} ", ticker)) {
                None => {

                }
                Some(_t) => {
                    tickers_in_title.insert(ticker);
                }
            }
        }
        return tickers_in_title;
    }).collect();
    println!("{:?}", tickers_in_each_title);
    // 3. Determine the weight of each of the posts talking about a given ticker
    // 4. Construct a portfolio fo stocks based on this initial weighting
    Ok(())
}
