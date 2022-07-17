#[macro_use]
extern crate rocket;

use std::fmt::Display;
use std::fs;

use rocket::request::FromParam;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

// The type to represent the ID of a message.
type Id = usize;

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
#[derive(Debug)]
struct Suffix<'r>(&'r str);

impl<'r> Display for Suffix<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//Adding some input parsing in so we can make sure we are only reading files from known values
//This also does not protect from injection attacks
impl<'r> FromParam<'r> for Suffix<'r> {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        if param == "1" || param == "2" {
            Ok(Suffix(param))
        } else {
            Err("You can only pass a 1 or 2 in here !")
        }
    }
}

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
            .mount("/cards", routes![get_suffix])
            .register("/cards", catchers![not_found])
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage())
}
