use std::collections::BTreeMap;

use clap::{Parser, Subcommand, ValueEnum};

use nektar::ThriftHiveMetastoreSyncClient;
use serde::Serialize;
extern crate thrift;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::cmds::catalogs::GetCatalogs;
use crate::cmds::{
    catalogs::{CreateCatalog, GetCatalog},
    databases::{GetDatabase, GetDatabases},
    partitions::{GetPartitionNamesByParts, GetPartitions},
    tables::{CreateTable, DropTable, GetTable, ListTables},
};

use crate::error::CliError;

pub type MetastoreClient = ThriftHiveMetastoreSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
    TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
>;

pub trait RunCommand<T: Serialize> {
    fn run(self, client: MetastoreClient) -> Result<T, CliError>;
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Thrift metastore endpoint, eg: host.com:9083
    metastore_url: String,
    #[arg(value_enum, long="format", default_value_t = Format::Json)]
    format: Format,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum Format {
    Json,
    #[cfg(feature = "yaml")]
    Yaml,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    GetTable(GetTable),
    GetCatalog(GetCatalog),
    GetCatalogs(GetCatalogs),
    GetPartitions(GetPartitions),
    GetPartitionNamesByParts(GetPartitionNamesByParts),
    GetDatabases(GetDatabases),
    GetDatabase(GetDatabase),
    CreateCatalog(CreateCatalog),
    CreateTable(CreateTable),
    ListTables(ListTables),
    DropTable(DropTable),
}

//TODO: refactor this, hopefully without type erasure
fn serialize<T: Serialize>(f: Format, v: Result<T, CliError>) -> Result<String, CliError> {
    match f {
        Format::Json => match &v {
            Ok(t) => Ok(serde_json::to_string(&t)?),
            Err(e) => Ok(serde_json::to_string(&BTreeMap::from([("error", e)]))?),
        },
        #[cfg(feature = "yaml")]
        Format::Yaml => match &v {
            Ok(t) => Ok(serde_yaml::to_string(&t)?),
            Err(e) => Ok(serde_yaml::to_string(&BTreeMap::from([("error", e)]))?),
        },
    }
}

impl Cli {
    fn client(&self) -> Result<MetastoreClient, CliError> {
        let mut c = TTcpChannel::new();
        c.open(&self.metastore_url)?;
        let (i_chan, o_chan) = c.split()?;
        let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
        let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

        Ok(ThriftHiveMetastoreSyncClient::new(i_prot, o_prot))
    }

    pub fn run(self) -> Result<String, CliError> {
        let client = self.client()?;
        match self.command {
            Commands::GetTable(get_table) => serialize(self.format, get_table.run(client)),
            Commands::GetCatalog(get_catalog) => serialize(self.format, get_catalog.run(client)),
            Commands::GetPartitions(get_partitions) => {
                serialize(self.format, get_partitions.run(client))
            }
            Commands::GetPartitionNamesByParts(get_parts) => {
                serialize(self.format, get_parts.run(client))
            }
            Commands::GetDatabase(get_database) => serialize(self.format, get_database.run(client)),
            Commands::GetDatabases(get_databases) => {
                serialize(self.format, get_databases.run(client))
            }
            Commands::CreateCatalog(create_catalog) => {
                serialize(self.format, create_catalog.run(client))
            }
            Commands::GetCatalogs(get_catalogs) => serialize(self.format, get_catalogs.run(client)),
            Commands::CreateTable(create_table) => serialize(self.format, create_table.run(client)),
            Commands::ListTables(list_tables) => serialize(self.format, list_tables.run(client)),
            Commands::DropTable(drop_table) => serialize(self.format, drop_table.run(client)),
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
