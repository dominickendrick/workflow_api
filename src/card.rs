pub mod card {
    use rocket::request::FromParam;
    use rocket::serde::{Deserialize, Serialize};
    use std::fmt::Display;

    type Id = usize;

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Card {
        pub id: Id,
        pub title: String,
        pub state: String,
        pub author: String,
        pub editor: String,
        pub message: String,
    }
    #[derive(Debug)]
    pub struct Suffix<'r>(pub &'r str);

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
}
