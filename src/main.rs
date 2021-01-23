fn get_tickers() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    let tickers = get_tickers()?;
    let posts = get_wsb_top().await?;
    let v: serde_json::Value = serde_json::from_str(&posts)?;
    println!("{}", v["data"]["children"][0]["data"]["title"]);
    // 1. Download all the stock tickers
    // 2. Convert the stock tickers into a vector
    // 3. Download the reddit WSB data
    // 4. For each post / comment search for an occurnce of the stock tickers
    // 5. Create a ranking of each asset based on coverage in the Forum
    Ok(())
}
