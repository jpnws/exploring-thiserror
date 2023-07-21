#![allow(dead_code)]

use std::error::Error;
use std::io;
use std::io::ErrorKind::ConnectionAborted;

fn connect_data_store() -> Result<(), io::Error> {
    Err(io::Error::from(ConnectionAborted))
}

fn get_data_for_key() -> Result<(), String> {
    Err("<secret-key>".to_string())
}

fn read_header() -> Result<String, ()> {
    Ok("text/html".to_string())
}

fn unknown() -> Result<(), bool> {
    Err(false)
}

#[derive(thiserror::Error)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

impl std::fmt::Debug for DataStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n    Error: {}", self)?;
        let mut current = self.source();
        while let Some(cause) = current {
            writeln!(f, "    Caused by:\n        {}", cause)?;
            current = cause.source();
        }
        Ok(())
    }
}

fn show_user_info() -> Result<(), DataStoreError> {
    connect_data_store()?;
    get_data_for_key().map_err(DataStoreError::Redaction)?;
    if let Ok(content_type) = read_header() {
        if content_type != "application/json" {
            return Err(DataStoreError::InvalidHeader {
                expected: "application/json".to_string(),
                found: content_type,
            });
        }
    }
    unknown().map_err(|_| DataStoreError::Unknown)?;
    Ok(())
}

fn main() {
    show_user_info().unwrap();
    // show_user_info().unwrap_or_else(|err| println!("{}", err));
}
