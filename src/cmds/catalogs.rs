use crate::cli::MetastoreClient;
use crate::cli::RunCommand;
use crate::error::CliError;
use clap::Args;
use nektar::Catalog;
use nektar::CreateCatalogRequest;
use nektar::GetCatalogsResponse;
use nektar::{GetCatalogRequest, GetCatalogResponse, TThriftHiveMetastoreSyncClient};

/// Get a single catalog by name
#[derive(Debug, Args)]
pub struct GetCatalog {
    name: String,
}

impl Into<GetCatalogRequest> for GetCatalog {
    fn into(self) -> GetCatalogRequest {
        GetCatalogRequest {
            name: Some(self.name),
        }
    }
}

impl RunCommand<GetCatalogResponse> for GetCatalog {
    fn run(self, mut client: MetastoreClient) -> Result<GetCatalogResponse, CliError> {
        Ok(client.get_catalog(self.into())?)
    }
}

/// Get a list of all catalogs in metastore
#[derive(Debug, Args)]
pub struct GetCatalogs;

impl RunCommand<GetCatalogsResponse> for GetCatalogs {
    fn run(self, mut client: MetastoreClient) -> Result<GetCatalogsResponse, CliError> {
        Ok(client.get_catalogs()?)
    }
}

/// Create a catalog
#[derive(Debug, Args)]
pub struct CreateCatalog {
    name: String,
    location_uri: Option<String>,
    #[arg(long = "description", short = 'd')]
    description: Option<String>,
}

impl Into<Catalog> for CreateCatalog {
    fn into(self) -> Catalog {
        Catalog {
            name: Some(self.name),
            description: self.description,
            location_uri: self.location_uri,
        }
    }
}

impl Into<CreateCatalogRequest> for CreateCatalog {
    fn into(self) -> CreateCatalogRequest {
        CreateCatalogRequest {
            catalog: Some(self.into()),
        }
    }
}

impl RunCommand<()> for CreateCatalog {
    fn run(self, mut client: MetastoreClient) -> Result<(), CliError> {
        Ok(client.create_catalog(self.into())?)
    }
}
