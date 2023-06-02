use clap::Parser;

extern crate thrift;

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel};

use nektar::{ThriftHiveMetastoreSyncClient, TThriftHiveMetastoreSyncClient, Table};

#[derive(Parser)]
struct Arg {
    metastore_uri: String
}

fn main() {
    match run() {
        Ok(t) => {
            println!("client ran successfully");
            println!("{:?}", t);
        }
        Err(e) => {
            println!("client failed with {:?}", e);
            std::process::exit(1);
        }
    }
}


fn run() -> thrift::Result<Vec<Table>> {

    let arguments = Arg::parse();
    let mut c = TTcpChannel::new();
    c.open(arguments.metastore_uri)?;
    let (i_chan, o_chan) = c.split()?;     // build the input/output protocol
    let i_prot = TBinaryInputProtocol::new(i_chan, true);
    let o_prot = TBinaryOutputProtocol::new(o_chan, true);

    // use the input/output protocol to create a Thrift client
    let mut client = ThriftHiveMetastoreSyncClient::new(i_prot, o_prot);
    return client.get_table_objects_by_name("hades".to_string(), vec!["MetadataEntities.TrackExtended.gcs".to_string()]);
}
