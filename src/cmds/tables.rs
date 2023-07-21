use clap::Args;
use crate::error::CliError;
use crate::cli::RunCommand;
use nektar::{TThriftHiveMetastoreSyncClient};
use crate::cli::MetastoreClient;
use serde_json;

#[derive(Debug, Args)]
pub struct GetTable {
    database:String,
    table:Vec<String>,
}

impl RunCommand for GetTable {

    fn run(self, mut client:MetastoreClient) -> Result<(), CliError> {
        let tables = client.get_table_objects_by_name(self.database, self.table)?;
        if let Ok(json) = serde_json::to_string(&tables) {
            println!("{}", json) 
        };
        Ok(())
    }
}
