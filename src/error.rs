use std::error::Error;

use serde::{ser::SerializeStruct, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Could not connect to metastore at {0}")]
    MetastoreUnavailble(String),
    #[error(transparent)]
    ThriftError(#[from] thrift::Error),
    #[error("Could not serialize to json")]
    JsonSerdeError(#[from] serde_json::Error),
    #[error("Could not serialize to json")]
    YamlSerdeError(#[from] serde_yaml::Error),
}

impl Serialize for CliError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CliError::MetastoreUnavailble(_msg) => serializer.serialize_str(&self.to_string()),
            CliError::ThriftError(e) => {
                let mut s = serializer.serialize_struct("error", 1)?;
                s.serialize_field("message", &e.to_string())?;
                s.end()
            }
            CliError::JsonSerdeError(e) => serializer.serialize_str(&e.to_string()),
            CliError::YamlSerdeError(e) => serializer.serialize_str(&e.to_string()),
        }
    }
}
