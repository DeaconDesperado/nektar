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
    databases::GetDatabases,
    partitions::{GetPartitionNamesByParts, GetPartitions},
    tables::GetTable,
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

#[derive(ValueEnum, Debug, Clone)]
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
    CreateCatalog(CreateCatalog),
}

fn serialize<T: Serialize>(f: Format, v: T) -> Result<String, CliError> {
    match f {
        Format::Json => Ok(serde_json::to_string(&v)?),
        #[cfg(feature = "yaml")]
        Format::Yaml => Ok(serde_yaml::to_string(&v)?),
    }
}

impl Cli {
    pub fn run(self) -> Result<String, CliError> {
        let mut c = TTcpChannel::new();
        c.open(&self.metastore_url)?;
        let (i_chan, o_chan) = c.split()?;
        let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
        let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

        let client = ThriftHiveMetastoreSyncClient::new(i_prot, o_prot);

        match self.command {
            Commands::GetTable(get_table) => serialize(self.format, get_table.run(client)?),
            Commands::GetCatalog(get_catalog) => serialize(self.format, get_catalog.run(client)?),
            Commands::GetPartitions(get_partitions) => {
                serialize(self.format, get_partitions.run(client)?)
            }
            Commands::GetPartitionNamesByParts(get_parts) => {
                serialize(self.format, get_parts.run(client)?)
            }
            Commands::GetDatabases(get_databases) => {
                serialize(self.format, get_databases.run(client)?)
            }
            Commands::CreateCatalog(create_catalog) => {
                serialize(self.format, create_catalog.run(client)?)
            }
            Commands::GetCatalogs(get_catalogs) => {
                serialize(self.format, get_catalogs.run(client)?)
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
