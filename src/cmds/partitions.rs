use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use nektar::TThriftHiveMetastoreSyncClient;

/// Get the partitions of a table
#[derive(Debug, Args)]
pub struct GetPartitions {
    database: String,
    table: String,
}

impl RunCommand<Vec<String>> for GetPartitions {
    fn run(self, mut client: MetastoreClient) -> Result<Vec<String>, CliError> {
        Ok(client.get_partition_names(self.database, self.table, 10)?)
    }
}

/// Get the partitions of a table by partition value
#[derive(Debug, Args)]
pub struct GetPartitionNamesByParts {
    database: String,
    table: String,
    part_vals: Vec<String>,
    #[arg(long, default_value_t = 1)]
    max_parts: i16,
}

impl RunCommand<Vec<String>> for GetPartitionNamesByParts {
    fn run(self, mut client: MetastoreClient) -> Result<Vec<String>, CliError> {
        Ok(client.get_partition_names_ps(
            self.database,
            self.table,
            self.part_vals,
            self.max_parts,
        )?)
    }
}
