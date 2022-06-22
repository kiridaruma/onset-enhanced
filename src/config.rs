use serde::Deserialize;
use std::io;
use std::io::Read;
use std::{fs::File, path::Path, path::PathBuf};
use toml;

#[derive(Deserialize)]
pub struct Config {
    data_dir: PathBuf,
    bcdice_url: String,
    room_count_limit: u32,
    room_name_limit: u32,
    room_delete_interval_sec: u32,
    message_length_limit: u32,
    nickname_length_limit: u32,
}

pub enum Error {
    LoadFailed(io::Error),
    ParseFailed(toml::de::Error),
}

impl Config {
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn bcdice_url(&self) -> &str {
        &self.bcdice_url
    }

    pub fn room_count_limit(&self) -> &u32 {
        &self.room_count_limit
    }

    pub fn room_name_limit(&self) -> &u32 {
        &self.room_name_limit
    }

    pub fn room_delete_interval_sec(&self) -> &u32 {
        &self.room_delete_interval_sec
    }

    pub fn message_length_limit(&self) -> &u32 {
        &self.message_length_limit
    }

    pub fn nickname_length_limit(&self) -> &u32 {
        &self.nickname_length_limit
    }

    pub fn load(path: &Path) -> Result<Self, Error> {
        let file_text = Self::load_file(path).map_err(|e| Error::LoadFailed(e))?;

        toml::from_str(file_text.as_str()).map_err(|e| Error::ParseFailed(e))
    }

    fn load_file(path: &Path) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut file_text = String::new();
        file.read_to_string(&mut file_text)?;

        Ok(file_text)
    }
}

#[cfg(test)]
mod test {
    use crate::config::{Config, Error};
    use std::fs::remove_file;
    use std::path::Path;
    use std::{fs::File, io::Write};

    #[test]
    fn load_success() {
        let valid_toml_text = String::from(
            r#"
##################################
# Onset! Enhanced config samples #
##################################

# directory which saved onset data
data_dir="/path/to/data_dir/"

# bcdice-api endpoint url
# see also https://github.com/bcdice/bcdice-api
bcdice_url="http://example.com/bcdice-api/endpoint"

# room settings
room_count_limit=100
room_name_limit=30
room_delete_interval_sec=86400

# nickname and text settings
message_length_limit=5000
nickname_length_limit=20
        "#,
        );

        let file_path = Path::new("./config_valid_toml_test.toml");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(valid_toml_text.as_bytes()).unwrap();
        let result = Config::load(&file_path);

        assert_eq!(true, result.is_ok());
        remove_file(file_path).unwrap();
    }

    #[test]
    fn failed_file_not_exists() {
        let not_exists_file_path = Path::new("./not_exists");
        let result = Config::load(&not_exists_file_path);
        assert_eq!(true, result.is_err());
        match result {
            Err(Error::LoadFailed(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn failed_parse_toml() {
        let invalid_toml_text = String::from("The niece of time");
        let file_path = Path::new("./config_invalid_toml_test.toml");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(invalid_toml_text.as_bytes()).unwrap();
        let result = Config::load(&file_path);

        assert_eq!(true, result.is_err());
        match result {
            Err(Error::ParseFailed(_)) => assert!(true),
            _ => assert!(false),
        }
        remove_file(file_path).unwrap();
    }
}
