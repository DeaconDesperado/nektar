use clap::Args;
use crate::error::CliError;
use crate::cli::RunCommand;
use nektar::{TThriftHiveMetastoreSyncClient};
use crate::cli::MetastoreClient;
use serde_json;

#[derive(Debug, Args)]
pub struct GetPartitions {
    database:String,
    table:String,
}

impl RunCommand for GetPartitions {

    fn run(self, mut client:MetastoreClient) -> Result<(), CliError> {
        let partitions = client.get_partition_names(self.database, self.table, 10)?;
        if let Ok(json) = serde_json::to_string(&partitions) {
            println!("{}", json) 
        };
        Ok(())
    }
}
