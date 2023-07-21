use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Could not connect to metastore at {0}")]
    MetastoreUnavailble(String)
}
