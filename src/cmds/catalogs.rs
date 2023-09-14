use clap::Args;
use crate::error::CliError;
use crate::cli::RunCommand;
use nektar::{TThriftHiveMetastoreSyncClient, GetCatalogRequest};
use crate::cli::MetastoreClient;
use serde_json;

#[derive(Debug, Args)]
pub struct GetCatalog {
    name:String
}

impl RunCommand for GetCatalog {

    fn run(self, mut client:MetastoreClient) -> Result<(), CliError> {
        match client.get_catalog(GetCatalogRequest{ name: Some(self.name) }) {
           Ok(catalog) =>  
            match serde_json::to_string(&catalog) {
                Ok(json) => { 
                    println!("{}", json);
                    Ok(())
                },
                Err(e) => Err(CliError::SerdeError(e))
            },
           Err(e) => Err(CliError::ThriftError(e))
        }
    }
}
