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
    // 1. Pull all the stock tickers and convert to a vec
    let tickers = get_tickers()?;
    // 2. Download and parse the reddit WSB data
    let posts = get_wsb_top().await?;
    let v: serde_json::Value = serde_json::from_str(&posts)?;
    println!("{}", v["data"]["children"][0]["data"]["title"]);
    // 3. Determine the weight of each of the posts talking about a given ticker
    // 4. Construct a portfolio fo stocks based on this initial weighting
    Ok(())
}
