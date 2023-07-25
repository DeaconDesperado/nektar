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

#[derive(Debug, Args)]
pub struct GetPartitionNamesByParts {
    database:String,
    table:String,
    part_vals:Vec<String>,
    #[arg(long, default_value_t = 1)]
    max_parts:i16
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

impl RunCommand for GetPartitionNamesByParts {

    fn run(self, mut client:MetastoreClient) -> Result<(), CliError> {
        let partitions = client.get_partition_names_ps(self.database, self.table, self.part_vals, self.max_parts)?;
        if let Ok(json) = serde_json::to_string(&partitions) {
            println!("{}", json);
        };
        Ok(())
    }

}
