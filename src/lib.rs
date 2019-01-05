//#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod reader {
    use std::fs;

    pub fn get_users() -> String {
        let contents = fs::read_to_string("users.txt").expect("Something went wrong reading the file");
        contents
    }
    pub fn get_key() -> String {
        let contents = fs::read_to_string("key.txt").expect("Something went wrong reading the file");
        contents
    }
}
pub mod communicator {
    use serde_json::Value;

    pub fn get_chart(user: &str, key: &str) -> Result<Value, reqwest::Error> {
        let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getWeeklyAlbumChart&user={}&api_key={}&format=json",
                              user, key);
        let mut response = reqwest::get(&request_url)?;

        let answer: Value = response.json()?;

        Ok(answer)
    }

    pub fn parse_album(data: &Value) -> (String, i64) {
        (
            String::from(data["name"].as_str().unwrap()),
            data["playcount"].as_str().unwrap().parse().unwrap(),
        )
    }
}

pub mod drawer {}
