use serde::Deserialize;
use std::env;
use std::io::{BufReader, Read};
use std::fs::{self, DirEntry};
use std::sync::OnceLock;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidCwd(std::io::Error),
    NoConfigFileFound,
    CouldNotParseConfig(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::InvalidCwd(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::CouldNotParseConfig(value)
    }
}


impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}


#[derive(Debug, Deserialize)]
pub struct Build {
    pub out_folder: String,
    pub assets_folder: String,
    pub templates_folder: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}


#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
    pub title: String,
    pub description: String,
    pub build: Build,
    pub server: Server,
}

impl Config {
    pub fn get_static() -> &'static Config {
        static INSTANCE: OnceLock<Config> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            Config::acquire_in_cdr().unwrap_or_else(|exception| {
                panic!("Failed to load config from current directory: {exception:?}");
            })
        })
    }
    /// Search for a valid configuration file in a given directory.
    ///
    /// Run against a directory (NOTE: can use `env::current_dir()`)
    /// Attempt to read the directory.
    /// Attempt to find a file with the expected name of the config file.
    /// Open the file by reading it into a buffer, and then to a String.
    /// Attempt to deserialze it.
    pub fn acquire(dir: std::path::PathBuf) -> Result<Config> { 
        let possible_location = locate_config_file_entry(dir)?;
        let possible_file = fs::File::open(possible_location.path())?;
        let buf_reader = BufReader::new(possible_file);
        serde_json::from_reader(buf_reader).map_err(|err| Error::CouldNotParseConfig(err))
    }

    pub fn acquire_in_cdr() -> Result<Config> {
        let cdr = env::current_dir()?;
        Self::acquire(cdr)
    }
}

fn locate_config_file_entry(dir: std::path::PathBuf) -> Result<DirEntry> {
    fs::read_dir(dir)
            .expect("Unable to read current directory")
            .flat_map(|e| e)
            .find(|e| e.file_name() == "shakyrave")
            .ok_or(Error::NoConfigFileFound)
}
