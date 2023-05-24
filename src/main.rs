#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;

use reqwest;
use scraper::{Html, Selector};
use rocket::serde::json::Json;
use serde::Serialize;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions};

#[derive(Serialize)]
struct Item {
    amount_of_items: String,
    starting_at: String,
    lowest_price: String,
    item: String,
    game: String,
    image_url: String,
}

#[get("/items")]
async fn items() -> Json<Vec<Item>> {
    let resp = reqwest::get("https://steamcommunity.com/market/").await.unwrap();
    let body = resp.text().await.unwrap();

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a.market_listing_row_link").unwrap();

    let mut items = Vec::new();

    for element in fragment.select(&selector) {
        let text: Vec<_> = element.text().collect();
        let cleaned_data: Vec<String> = text.into_iter()
            .map(|item| item.replace("\n", "").replace("\t", ""))
            .filter(|item| !item.is_empty())
            .collect();
        let image_element = element.select(&Selector::parse("img").unwrap()).next();
        let image_url = image_element.map_or_else(|| "".to_string(), |img| img.value().attr("src").unwrap_or("").to_string());
        if cleaned_data.len() >= 6 {
            let item = Item {
                amount_of_items: cleaned_data[0].clone(),
                starting_at: cleaned_data[2].clone(),
                lowest_price: cleaned_data[3].clone(),
                item: cleaned_data[4].clone(),
                game: cleaned_data[5].clone(),
                image_url, // new field
            };
            items.push(item);
        }
    }

    Json(items)
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000", // replace with the origin of your frontend
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");

    rocket::build()
        .mount("/", routes![items])
        .attach(cors)
}
