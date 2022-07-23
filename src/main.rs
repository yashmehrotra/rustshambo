#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use chrono;

use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
use rust_embed::RustEmbed;

use rocket::http::Method;
use rocket_cors;

use std::borrow::Cow;

#[derive(Deserialize)]
struct UserInput {
    session_id: u64,
    hand: String,
}

#[derive(Serialize)]
struct AIResponse {
    ai_hand: String,
}

#[derive(RustEmbed)]
#[folder = "frontend/"]
struct Asset;

#[rocket::get("/")]
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

#[rocket::post("/compute", format = "json", data = "<user_input>")]
fn compute(user_input: Json<UserInput>) -> Json<AIResponse> {

    let ai_hand:&str = match user_input.hand.to_lowercase().as_str() {
        "rock" => "paper",
        "paper" => "scissor",
        "scissor" => "rock",
        _ => "invalid hand",
    };

    println!("[{:?}] 200 - {}", chrono::offset::Local::now(), user_input.session_id);
    Json(AIResponse{ai_hand: ai_hand.to_string()})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        ..rocket_cors::CorsOptions::default()
    }.to_cors().unwrap();


    let _rocket = rocket::build().attach(cors)
        .mount("/", rocket::routes![index, compute])
        .launch()
        .await?;

    Ok(())
}
