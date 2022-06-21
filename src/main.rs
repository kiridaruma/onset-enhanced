mod config;

use std::{env, path::PathBuf};

use crate::config::{Config, Error};

fn main() {
    let args = env::args();
    if args.len() < 2 {
        unusual_exit("not enough arguments")
    }
    let arg_list: Vec<String> = args.collect();
    let path = arg_list
        .get(1)
        .map(|path_str| PathBuf::from(path_str))
        .and_then(|path| if path.is_file() { Some(path) } else { None })
        .unwrap_or_else(|| unusual_exit("config file does not exist"));

    let config = Config::load(&path).unwrap_or_else(|err| match err {
        Error::LoadFailed(_) => unusual_exit("could not read config file"),
        Error::ParseFailed(_) => unusual_exit("config file is not toml format. check it"),
    });

    println!("Hello, world!");
}

pub fn unusual_exit(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(1);
}
