use crate::built_info::{PKG_HOMEPAGE, PKG_NAME, PKG_VERSION};
use gazelle_api::{GazelleClient, GazelleClientFactory};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct GazelleClientOptions {
    pub name: String,
    pub key: String,
    pub url: String,
    pub user: u32,
}

impl GazelleClientOptions {
    fn from_file(path: &Path) -> Result<Vec<Self>, Error> {
        let file = File::open(path).map_err(|e| Error {
            action: "create client".to_owned(),
            message: e.to_string(),
            domain: Some("file system".to_owned()),
            ..Error::default()
        })?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| Error {
            action: "create client".to_owned(),
            message: e.to_string(),
            domain: Some("deserialization".to_owned()),
            ..Error::default()
        })
    }

    pub(crate) fn get_client(&self) -> GazelleClient {
        let factory = GazelleClientFactory {
            key: self.key.clone(),
            url: self.url.clone(),
            user_agent: format!("{PKG_NAME}/{PKG_VERSION} ({PKG_HOMEPAGE})"),
        };
        factory.create()
    }
}

pub(crate) fn get_options() -> Result<Vec<GazelleClientOptions>, Error> {
    GazelleClientOptions::from_file(&PathBuf::from("config.yml"))
}