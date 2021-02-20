use serde::{Deserialize, Serialize};

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

const WALLSTREET_BETS_URL: &str = "https://www.reddit.com/r/wallstreetbets/new/.json?limit=100&t=month";

async fn get_wsb_top() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(WALLSTREET_BETS_URL)
        .await?
        .text()
        .await?;
    Ok(body)
}

pub async fn get_post_titles() ->  Result<Vec<String>, Box<dyn std::error::Error>> {
    let posts: RedditContainer<RedditListing> = serde_json::from_str(
        &get_wsb_top().await?
    )?;
    Ok(posts.data.children.iter().map(|p| { p.data.title.clone() }).collect())
}

