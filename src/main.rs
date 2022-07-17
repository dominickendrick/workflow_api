#[macro_use] extern crate rocket;

use std::fs;

use std::error::Error;
use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

// The type to represent the ID of a message.
type Id = usize;

// We're going to store all of the messages here. No need for a DB.
type CardList = Mutex<Vec<String>>;
type Cards<'r> = &'r State<CardList>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Card {
    id: Id,
    title: String,
    state: String,
    author: String,
    editor: String,
    message: String,
}

#[get("/", format = "html")]
fn get() -> Json<Vec<Card>> {

    let filename = "src/data/fixtures.json";
    //load file from disk ! This is obvs not the best thing to do for peformance ...
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    //parse the input json
    let cards: Vec<Card> = rocket::serde::json::from_str(contents.as_str()).unwrap();

    //serialise back to json in response
    Json(cards)
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
        rocket.mount("/cards", routes![get])
            .register("/cards", catchers![not_found])
            .manage(CardList::new(vec![]))
    })
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(stage())
}

