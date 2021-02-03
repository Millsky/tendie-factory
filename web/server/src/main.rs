use askama::Template;

struct TickerData<'a> {
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

struct PortfolioItem<'a> {
    meta: TickerData<'a>,
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
        portfolio_weight: 0.5555
    }]);
    assert_eq!(rendered_html.contains("GYZ"), true);
}

fn main() {
    println!("{}", render_portfolio(vec![PortfolioItem {
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
        portfolio_weight: 0.5555
    }]));
}
