use thiserror::Error;


#[derive(Error, Debug)]
pub enum CliError {
    #[error("Could not connect to metastore at {0}")]
    MetastoreUnavailble(String),
    #[error("Thrift error {0}")]
    ThriftError(#[from] thrift::Error)

}
