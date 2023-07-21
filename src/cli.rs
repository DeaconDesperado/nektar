use clap::{Parser, Subcommand};

use std::process::ExitCode;

extern crate thrift;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TIoChannel, TTcpChannel, ReadHalf, WriteHalf};
use nektar::{TThriftHiveMetastoreSyncClient, ThriftHiveMetastoreSyncClient};

use console::style;
use crate::error::CliError;
use crate::cmds:: {
    tables::GetTable
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
}


impl Cli {
    pub fn run(self) -> ExitCode {
        let mut c = TTcpChannel::new();
        c.open(self.metastore_uri).unwrap();
        let (i_chan, o_chan) = c.split().unwrap(); 
        let i_prot = TBinaryInputProtocol::new(i_chan, true);
        let o_prot = TBinaryOutputProtocol::new(o_chan, true);

        // use the input/output protocol to create a Thrift client
        let client = ThriftHiveMetastoreSyncClient::new(i_prot, o_prot);

        let output = match self.command {
            Commands::GetTable(get_table) => get_table.run(client)
        };
        match output {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{}", style(e).for_stderr().red()); 
                return ExitCode::FAILURE;
            }
        }
    }
}
