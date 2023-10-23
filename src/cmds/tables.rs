use crate::cli::Format;
use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use dialoguer::Confirm;
use nektar::{TThriftHiveMetastoreSyncClient, Table};
use std::convert::TryInto;
use std::path::PathBuf;
use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

/// Get a single table by database and table name
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

/// Create a table from a table definition file
#[derive(Debug, Args)]
pub struct CreateTable {
    /// The input format for the table definition file
    #[arg(value_enum, short='f', long="format", default_value_t = Format::Json)]
    format: Format,
    /// A file path for table definition
    table_definition_file: PathBuf,
}

impl RunCommand<Table> for CreateTable {
    fn run(self, mut client: MetastoreClient) -> Result<Table, CliError> {
        let now = SystemTime::now();
        let since_epoch: i32 = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .try_into()
            .unwrap();
        let file = File::open(self.table_definition_file)?;
        let mut table: Table = match self.format {
            Format::Json => serde_json::from_reader(file).unwrap(),
            #[cfg(feature = "yaml")]
            Format::Yaml => serde_yaml::from_reader(file).unwrap(),
        };
        table.create_time = Some(since_epoch);
        Ok(client.create_table(table.clone())?).map(|_| table)
    }
}

/// Drop a single table by database and table name
#[derive(Debug, Args)]
pub struct DropTable {
    db_name: String,
    table_name: String,
    #[arg(long = "delete")]
    delete: bool,
}

impl RunCommand<()> for DropTable {
    fn run(self, mut client: MetastoreClient) -> Result<(), CliError> {
        let confirmation = Confirm::new()
            .with_prompt(format!("Delete {}?", self.table_name))
            .interact()
            .unwrap();

        if confirmation {
            Ok(client.drop_table(self.db_name, self.table_name, self.delete)?)
        } else {
            Ok(())
        }
    }
}

/// Get tables in database with optional glob on name
#[derive(Debug, Args)]
pub struct ListTables {
    db_name: String,
    /// An optional glob search by table name
    #[arg(short = 's', long = "search", default_value = "*")]
    name_glob: String,
}

impl RunCommand<Vec<Table>> for ListTables {
    fn run(self, mut client: MetastoreClient) -> Result<Vec<Table>, CliError> {
        Ok(client
            .get_tables(self.db_name.to_owned(), self.name_glob.to_owned())
            .and_then(|tables| client.get_table_objects_by_name(self.db_name.to_owned(), tables))?)
    }
}
