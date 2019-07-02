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
        let mut words = line.split("=");
        let key = words.next().unwrap();
        let value = words.next().unwrap();
        match key {
            "x" => args.x = value.parse().unwrap(),
            "y" => args.y = value.parse().unwrap(),
            "period" => args.period = String::from(value),
            "captions" => args.captions = value.parse().unwrap(),
            "web" => args.web = value.parse().unwrap(),
            "user" => args.nick = if value == "" {None} else {Some(String::from(value))},
            "read_path" => args.path_read = String::from(value),
            "write_path" => args.path_write  = String::from(value),
            "out_path" => args.path_out = String::from(value),
            "web_path" => args.path_web = String::from(value),
            _ => panic!("check your config file"),
        }
    }
    args
}

pub fn parse_args(args: Vec<String>, mut res: Args) -> Result<Args, ConfigErr> {
    let mut args = args.into_iter();
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-x" => res.x = args.next().ok_or(NoArgument)?.parse().ok().ok_or(U32ParseError)?,
            "-y" => res.y = args.next().ok_or(NoArgument)?.parse().ok().ok_or(U32ParseError)?,
            "-p" => res.period = args.next().ok_or(NoArgument)?,
            "-c" => res.captions = true,
            "-w" => res.web = true,
            "-s" => res.nick = Some(args.next().ok_or(NoArgument)?),
            _ => return Err(WrongOption),
        }
    }
    if !vec!["7day", "1month", "3month", "6month", "1year", "overall"].contains(&res.period.as_str()) {
        return Err(WrongPeriod);
    }
    Ok(res)
}
pub enum ConfigErr {
    NoArgument,
    U32ParseError,
    WrongOption,
    WrongPeriod,
}

pub struct Args {
    pub x: u32,
    pub y: u32,
    pub period: String,
    pub captions: bool,
    pub nick: Option<String>,
    pub web: bool,
    pub path_write: String,
    pub path_read: String,
    pub path_out: String,
    pub path_web: String,
}
impl Args {
    fn new() -> Args {
        Args{ x: 5u32, y: 5u32, period: String::from("7day"), captions: false, nick: None, web: false, path_read: String::from("./data/"), path_write: String::from("./data/"), path_out: String::from(""), path_web: String::from("./docs/")}
    }

    pub fn load_users(&self) -> Vec<String> {
        match &self.nick {
            None => super::get_users(&self.path_read),
            Some(nick) => vec![nick.clone()],
        }
    }

    pub fn size(&self) -> usize {
        (self.x * self.y) as usize
    }

    pub fn get_key(&self) -> String {
        super::get_key(&self.path_read)
    }
}
