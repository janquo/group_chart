#![feature(try_trait)]

//#[macro_use]
extern crate image;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod reader {
    use std::fs;

    pub fn get_users() -> String {
        let contents = fs::read_to_string("users.txt")
            .expect("Something went wrong reading the users.txt file");
        contents
    }
    pub fn get_key() -> String {
        let contents =
            fs::read_to_string("key.txt").expect("Something went wrong reading the key.txt file");
        contents
    }
}
pub mod communicator {
    use serde_json::Value;
    use std::fs::File;

    pub fn get_chart(user: &str, key: &str) -> Result<Value, reqwest::Error> {
        let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period=7day&format=json",
                              user, key);
        let mut response = reqwest::get(&request_url)?;

        let answer: Value = response.json()?;

        Ok(answer)
    }

    pub fn parse_album(data: &Value) -> (String, String, i64) {
        eprintln!("{:?}", data);
        (
            String::from(data["name"].as_str().unwrap()),
            String::from(data["artist"]["name"].as_str().unwrap()),
            data["playcount"].as_i64().unwrap(),
        )
    }

    pub fn get_album_inf(name: &str, artist: &str, key: &str) -> Result<Value, reqwest::Error> {
        let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, artist, name);
        let mut response = reqwest::get(&request_url)?;

        let answer: Value = response.json()?;

        Ok(answer)
    }

    pub fn get_img_url(data: &Value) -> Result<String, std::option::NoneError> {
            Ok(String::from(data["album"]["image"].as_array()?[1]["#text"].as_str()?))
    }

    pub fn download_image(target: &str) -> Result<String, reqwest::Error> {
        let mut response = reqwest::get(target)?;
        let mut result = String::new();

        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");
            let fname = format!("./images/{}", fname);
            let fname = fname.as_str();
            result = String::from(fname);
            File::create(fname).unwrap()
        };
        std::io::copy(&mut response, &mut dest).unwrap();
        Ok(result)
    }
}

pub mod drawer {

    pub fn play() {
        use image::GenericImage;

        let mut img = image::ImageBuffer::new(128, 64);

        let img2 = image::open("img.png").unwrap();

        img.copy_from(&img2, 0, 0);
        img.copy_from(&img2, 64, 0);

        img.save("test.png").unwrap();
    }

    pub fn collage(images: Vec<String>) {
        use image::GenericImage;

        let mut img = image::ImageBuffer::new(320, 320);

        for i in 0..25 as usize {
            let img2 = image::open(images[i].clone()).unwrap();

            img.copy_from(&img2, 64*(i % 5) as u32, 64*(i / 5) as u32);
        }

        img.save("test.png").unwrap();
    }
}
