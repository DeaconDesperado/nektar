use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use nektar::TThriftHiveMetastoreSyncClient;

/// Get all databases in the metastore
#[derive(Debug, Args)]
pub struct GetDatabases;

impl RunCommand<Vec<String>> for GetDatabases {
    fn run(self, mut client: MetastoreClient) -> Result<Vec<String>, CliError> {
        Ok(client.get_all_databases()?)
    }
}
