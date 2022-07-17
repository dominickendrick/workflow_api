#[macro_use]
extern crate rocket;

use std::fmt::Display;
use std::fs;

use rocket::request::FromParam;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use workflow_api::card::card::{Card, Suffix};

pub mod card;

// The type to represent the ID of a message.

#[get("/<suffix>")]
fn get_suffix(suffix: Suffix) -> Json<Vec<Card>> {
    let filename = format!("src/data/fixture{}.json", suffix.0);

    //load file from disk ! This is obvs not the best thing to do for peformance ...
    let contents = fs::read_to_string(&filename)
        .expect(format!("This failed looking for {} ", &filename).as_str());

    //parse the input json
    //TODO: handle parse errors correctly
    let cards: Vec<Card> = rocket::serde::json::from_str(contents.as_str()).unwrap();

    //serialise back to json in response
    Json(cards)
}

#[put("/merge", data = "<cards>")]
fn merge(cards: Json<Vec<Card>>) -> Json<Vec<Card>> {
    let filename = "src/data/fixture1.json";

    //load file from disk ! This is obvs not the best thing to do for peformance ...
    let contents = fs::read_to_string(&filename)
        .expect(format!("This failed looking for {} ", &filename).as_str());

    let cards1: Vec<Card> = rocket::serde::json::from_str(contents.as_str()).unwrap();

    //serialise back to json in response
    cards
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket
            .mount("/cards", routes![get_suffix, merge])
            .register("/cards", catchers![not_found])
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage())
}
