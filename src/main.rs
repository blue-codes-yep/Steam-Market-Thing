use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://steamcommunity.com/market/").await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a.market_listing_row_link").unwrap();

    for element in fragment.select(&selector) {
        let text: Vec<_> = element.text().collect();
        let cleaned_data: Vec<String> = text.into_iter()
            .map(|item| item.replace("\n", "").replace("\t", ""))
            .filter(|item| !item.is_empty())
            .collect();
        if cleaned_data.len() >= 6 {
            let formatted_data = vec![
                "Amount of items:".to_string(),
                cleaned_data[0].clone(),
                "Starting at:".to_string(),
                cleaned_data[2].clone(),
                "Lowest Price:".to_string(),
                cleaned_data[3].clone(),
                "Item:".to_string(),
                cleaned_data[4].clone(),
                "Game:".to_string(),
                cleaned_data[5].clone(),
            ];
            println!("{:?}", formatted_data);
        }
    }

    Ok(())
}