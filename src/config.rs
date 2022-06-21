use serde::Deserialize;
use std::io;
use std::io::Read;
use std::{fs::File, path::PathBuf};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
    pub bcdice_url: String,
    pub room_count_limit: u32,
    pub room_name_limit: u32,
    pub room_delete_interval_sec: u32,
    pub message_length_limit: u32,
    pub nickname_length_limit: u32,
}

pub enum Error {
    LoadFailed(io::Error),
    ParseFailed(toml::de::Error),
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, Error> {
        let file_text = Self::load_file(path).map_err(|e| Error::LoadFailed(e))?;

        toml::from_str(file_text.as_str()).map_err(|e| Error::ParseFailed(e))
    }

    fn load_file(path: &PathBuf) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut file_text = String::new();
        file.read_to_string(&mut file_text)?;

        Ok(file_text)
    }
}
