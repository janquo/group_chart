use super::Args;
use std::fs;
use std::path::PathBuf;
use ConfigErr::*;

pub fn load_args() -> Vec<String> {
    std::env::args().collect()
}

pub fn load() -> Args {
    let mut args = Args::new();
    let lines = std::fs::read_to_string("config.ini")
        .expect("Something went wrong reading the config.ini file");
    let lines = lines.lines();
    for line in lines.into_iter() {
        let mut words = line.split('=');
        let key = words.next().unwrap();
        let value = words.next().unwrap();
        match key {
            "x" => args.x = value.parse().unwrap(),
            "y" => args.y = value.parse().unwrap(),
            "period" => args.period = String::from(value),
            "captions" => args.captions = value.parse().unwrap(),
            "web" => args.web = value.parse().unwrap(),
            "user" => {
                args.nick = if value.is_empty() {
                    None
                } else {
                    Some(String::from(value))
                }
            }
            "read_path" => args.path_read = PathBuf::from(value),
            "write_path" => args.path_write = PathBuf::from(value),
            "out_path" => args.path_out = PathBuf::from(value),
            "web_path" => args.path_web = PathBuf::from(value),
            "tygodniowa" => args.save_history = value.parse().unwrap(),
            _ => panic!("check your config file"),
        }
    }
    args
}

#[derive(Debug)]
pub enum ConfigErr {
    NoArgument,
    U32ParseError,
    WrongOption,
    WrongPeriod,
}

impl Args {
    fn new() -> Args {
        Args {
            x: 5u32,
            y: 5u32,
            period: String::from("7day"),
            captions: false,
            nick: None,
            web: false,
            path_read: PathBuf::from("./data"),
            path_write: PathBuf::from("./data"),
            path_out: PathBuf::from(""),
            path_web: PathBuf::from("./docs"),
            save_history: false,
        }
    }

    pub fn parse(mut self, args: Vec<String>) -> Result<Args, ConfigErr> {
        let mut args = args.into_iter();
        args.next();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-x" => {
                    self.x = args
                        .next()
                        .ok_or(NoArgument)?
                        .parse()
                        .ok()
                        .ok_or(U32ParseError)?
                }
                "-y" => {
                    self.y = args
                        .next()
                        .ok_or(NoArgument)?
                        .parse()
                        .ok()
                        .ok_or(U32ParseError)?
                }
                "-p" => self.period = args.next().ok_or(NoArgument)?,
                "-c" => self.captions = true,
                "-w" => self.web = true,
                "-s" => self.nick = Some(args.next().ok_or(NoArgument)?),
                "-t" => self.save_history = true,
                _ => return Err(WrongOption),
            }
        }
        if !vec!["7day", "1month", "3month", "6month", "12month", "overall"]
            .contains(&self.period.as_str())
        {
            return Err(WrongPeriod);
        }
        Ok(self)
    }

    pub fn load_users(&self) -> Vec<String> {
        match &self.nick {
            None => super::reader::get_users(&self.path_read),
            Some(nick) => vec![nick.clone()],
        }
    }

    pub fn size(&self) -> usize {
        (self.x * self.y) as usize
    }

    pub fn get_key(&self) -> String {
        let key_path = self.path_read.join("key.txt");
        fs::read_to_string(&key_path)
            .unwrap_or_else(|_| panic!("Something went wrong reading {}", &key_path.display()))
    }

    pub fn get_spotify_auth(&self) -> (String, String) {
        let path = self.path_read.join("spotify.json");
        let file_content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Something went wrong reading {}", path.display()));
        let json: serde_json::Value = serde_json::from_str(&file_content).unwrap();
        (
            String::from(json["id"].as_str().unwrap()),
            String::from(json["secret"].as_str().unwrap()),
        )
    }

    pub fn placeholder_img(&self) -> PathBuf {
        self.path_read.join("blank.png")
    }
}
