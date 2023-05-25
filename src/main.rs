#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use dotenv::dotenv;
use reqwest;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use scraper::{Html, Selector};
use serde::Serialize;
use std::env;
use tokio_postgres::NoTls;
use tokio_postgres::types::ToSql;
use std::sync::Arc;

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
    dotenv().ok();
    let db_password = env::var("DB_PASS").expect("DB_PASS must be set");

    let resp = reqwest::get("https://steamcommunity.com/market/")
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let selector = Selector::parse("a.market_listing_row_link").unwrap();

    let elements_data: Vec<_> = {
        let fragment = Html::parse_document(&body);
        let elements: Vec<_> = fragment.select(&selector).collect();

        elements.iter().map(|element| {
            let text: Vec<_> = element.text().collect();
            let cleaned_data: Vec<String> = text.into_iter()
                .map(|item| item.replace("\n", "").replace("\t", ""))
                .filter(|item| !item.is_empty())
                .collect();
            let image_element = element.select(&Selector::parse("img").unwrap()).next();
            let image_url = image_element.map_or_else(|| "".to_string(), |img| img.value().attr("src").unwrap_or("").to_string());
            (cleaned_data, image_url)
        }).collect()
    };
    let mut items = Vec::new();

    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=localhost user=postgres password={} dbname=steam",
            db_password
        ),
        NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(client);
    for (cleaned_data, image_url) in elements_data {
        if cleaned_data.len() >= 6 {
            let amount_of_items = cleaned_data[0].clone();
            let starting_at = cleaned_data[2].clone();
            let lowest_price = cleaned_data[3].clone();
            let item_name = cleaned_data[4].clone();
            let game = cleaned_data[5].clone();
    
            let params: Vec<&(dyn ToSql + Sync)> = vec![
                &amount_of_items as &(dyn ToSql + Sync),
                &starting_at as &(dyn ToSql + Sync),
                &lowest_price as &(dyn ToSql + Sync),
                &item_name as &(dyn ToSql + Sync),
                &game as &(dyn ToSql + Sync),
                &image_url as &(dyn ToSql + Sync),
            ];
            
            let client = Arc::clone(&client);
            let result = client.execute(
                "INSERT INTO items (amount_of_items, starting_at, lowest_price, item, game, image_url) VALUES ($1, $2, $3, $4, $5, $6)",
                &params,
            ).await;
            match result {
                Ok(_) => {
                    let item = Item {
                        amount_of_items,
                        starting_at,
                        lowest_price,
                        item: item_name,
                        game,
                        image_url, // new field
                    };
                    items.push(item);
                },
                Err(e) => eprintln!("Failed to insert item into database: {}", e),
            }
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

    rocket::build().mount("/", routes![items]).attach(cors)
}
