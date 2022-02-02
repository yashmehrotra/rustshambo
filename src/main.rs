//#[macro_use] extern crate rocket;

#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use chrono;

use serde::Deserialize;
use rocket_contrib::json;
use rocket_contrib::json::{Json,JsonValue};

use rocket::http::Method;
use rocket_cors;

#[derive(Deserialize)]
struct UserInput {
    session_id: u64,
    hand: String,
}

#[rocket::get("/")]
fn index() -> JsonValue {
    json!({"hello": "world"})
}

#[rocket::post("/compute", format = "json", data = "<user_input>")]
fn compute(user_input: Json<UserInput>) -> JsonValue {

    let ai_hand:&str = match user_input.hand.to_lowercase().as_str() {
        "rock" => "paper",
        "paper" => "scissor",
        "scissor" => "rock",
        _ => "invalid hand",
    };

    println!("[{:?}] 200 - {}", chrono::offset::Local::now(), user_input.session_id);
    json!({"ai_hand": ai_hand})
}

fn main() {
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite().attach(cors.to_cors().unwrap())
        .mount("/", rocket::routes![index, compute])
        .launch();
}
