use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://steamcommunity.com/market/").await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a.market_listing_row_link").unwrap();

    for element in fragment.select(&selector) {
        let text = element.text().collect::<Vec<_>>();
        let cleaned_data: Vec<String> = text.into_iter()
            .map(|item| item.replace("\n", "").replace("\t", ""))
            .filter(|item| !item.is_empty())
            .collect();
        println!("{:?}", cleaned_data);
    }

    Ok(())
}
