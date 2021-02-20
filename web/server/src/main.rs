use askama::Template;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::{HashMap};
use serde::{Deserialize};

#[derive(Clone, Deserialize, Debug)]
struct TickerData<'a> {
    #[serde(borrow)]
    symbol: &'a str,
    company_name: &'a str,
    last_sale: &'a str,
    net_change: &'a str,
    net_percent_change: &'a str,
    market_capitalization: &'a str,
    country: &'a str,
    ipo_year: &'a str,
    volume: &'a str,
    sector: &'a str,
    industry: &'a str,
}

#[derive(Clone, Deserialize, Debug)]
struct PortfolioItem<'a> {
    #[serde(borrow)]
    meta: TickerData<'a>,
    occurrences: i32,
    ticker: String,
    titles_with_ticker: Vec<String>,
    sentiment: f64,
    portfolio_weight: f64,
}

#[derive(Template)]
#[template(path = "index.html")]
struct PortfolioTemplate<'a> {
stocks: Vec<PortfolioItem<'a>>,
}

fn render_portfolio (stocks: Vec<PortfolioItem>) -> String {
    let portfolio_html = PortfolioTemplate { stocks };
    portfolio_html.render().unwrap()
}

#[test]
fn renders_the_passed_in_ticker() {
    let rendered_html: String = render_portfolio(vec![PortfolioItem {
        meta: TickerData {
            symbol: "GYZ",
            company_name: "",
            last_sale: "",
            net_change: "",
            net_percent_change: "",
            market_capitalization: "",
            country: "",
            ipo_year: "",
            volume: "",
            sector: "",
            industry: ""
        },
        occurrences: 0,
        ticker: (),
        titles_with_ticker: (),
        sentiment: 0.0,
        portfolio_weight: 0.5555
    }]);
    assert_eq!(rendered_html.contains("GYZ"), true);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = fs::read_to_string("./portfolio.json")?;
    let portfolio: Vec<PortfolioItem> = serde_json::from_str(contents.as_str()).unwrap();
    let mut file = File::create("index.html")?;
    file.write_all(render_portfolio(portfolio).as_bytes())?;
    Ok(())
}
