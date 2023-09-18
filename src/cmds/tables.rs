use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use nektar::{TThriftHiveMetastoreSyncClient, Table};

#[derive(Debug, Args)]
pub struct GetTable {
    database: String,
    table: Vec<String>,
}

impl RunCommand<Vec<Table>> for GetTable {
    fn run(self, mut client: MetastoreClient) -> Result<Vec<Table>, CliError> {
        Ok(client.get_table_objects_by_name(self.database, self.table)?)
    }
}
