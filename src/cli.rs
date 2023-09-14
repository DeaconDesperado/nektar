use clap::{Parser, Subcommand};

extern crate thrift;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TIoChannel, TTcpChannel, ReadHalf, WriteHalf};
use nektar::ThriftHiveMetastoreSyncClient;

use crate::error::CliError;
use crate::cmds:: {
    tables::GetTable, partitions::{GetPartitions, GetPartitionNamesByParts},
    catalogs::GetCatalog
};

pub type MetastoreClient = ThriftHiveMetastoreSyncClient<TBinaryInputProtocol<ReadHalf<TTcpChannel>>, TBinaryOutputProtocol<WriteHalf<TTcpChannel>>>;

pub trait RunCommand {
    fn run(self, client: MetastoreClient) -> Result<(), CliError>;
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    metastore_uri: String,
    #[clap(subcommand)]
    command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    GetTable(GetTable), 
    GetCatalog(GetCatalog),
    GetPartitions(GetPartitions),
    GetPartitionNamesByParts(GetPartitionNamesByParts)
}


impl Cli {
    pub fn run(self) -> Result<(), CliError> {
        let mut c = TTcpChannel::new();
        c.open(self.metastore_uri)?;
        let (i_chan, o_chan) = c.split()?;
        let i_prot = TBinaryInputProtocol::new(i_chan, true);
        let o_prot = TBinaryOutputProtocol::new(o_chan, true);

        // use the input/output protocol to create a Thrift client
        let client = ThriftHiveMetastoreSyncClient::new(i_prot, o_prot);

        match self.command {
            Commands::GetTable(get_table) => get_table.run(client),
            Commands::GetCatalog(get_catalog) => get_catalog.run(client),
            Commands::GetPartitions(get_partitions) => get_partitions.run(client),
            Commands::GetPartitionNamesByParts(get_parts) => get_parts.run(client)
        }
    }
}
