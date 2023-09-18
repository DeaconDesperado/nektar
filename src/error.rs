use std::error::Error;

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Could not connect to metastore at {0}")]
    MetastoreUnavailble(String),
    #[error("Thrift err {0:?}")]
    ThriftError(#[from] thrift::Error),
    #[error("Could not serialize to json")]
    SerdeError(#[from] serde_json::Error),
}

impl Serialize for CliError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CliError::MetastoreUnavailble(_msg) => serializer.serialize_str(&self.to_string()),
            CliError::ThriftError(e) => serializer.serialize_str(&e.to_string()),
            CliError::SerdeError(e) => serializer.serialize_str(&e.to_string()),
        }
    }
}
