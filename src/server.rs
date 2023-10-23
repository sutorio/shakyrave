use std::env;
use std::sync::OnceLock;

use axum::Router;
use axum::http::StatusCode;
use tower_http::services::ServeDir;

use crate::config::Config;

// ----------------------------------------------------------------------------------
// Custom errors
// ----------------------------------------------------------------------------------

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Config(crate::config::Error)
}

impl From<crate::config::Error> for Error {
    fn from(value: crate::config::Error) -> Self {
        Self::Config(value)
    }
}


impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}


// ----------------------------------------------------------------------------------
// Core
// ----------------------------------------------------------------------------------

async fn serve() -> Result<()> {
    unimplemented!()
}


