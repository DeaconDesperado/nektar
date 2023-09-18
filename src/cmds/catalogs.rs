use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use nektar::{GetCatalogRequest, GetCatalogResponse, TThriftHiveMetastoreSyncClient};
use serde_json;

#[derive(Debug, Args)]
pub struct GetCatalog {
    name: String,
}

impl RunCommand<GetCatalogResponse> for GetCatalog {
    fn run(self, mut client: MetastoreClient) -> Result<GetCatalogResponse, CliError> {
        Ok(client.get_catalog(GetCatalogRequest {
            name: Some(self.name),
        })?)
    }
}
