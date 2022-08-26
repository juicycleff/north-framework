use async_trait::async_trait;
use std::collections::HashMap;

use serde_json::Value;

use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CAConfig {
    Provider: String,
    Config: Value,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARootList {
    ActiveRootID: String,
    TrustDomain: String,
    Roots: Vec<CARoot>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARoot {
    ID: String,
    Name: String,
    RootCert: String,
    Active: bool,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[allow(clippy::upper_case_acronyms)]
#[async_trait]
pub trait ConnectCA {
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)>;
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)>;
    async fn ca_set_config(
        &self,
        conf: &CAConfig,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
}

#[async_trait]
impl ConnectCA for Client {
    /// https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)> {
        get("/v1/connect/ca/roots", &self.config, HashMap::new(), q).await
    }

    /// https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)> {
        get(
            "/v1/connect/ca/configuration",
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn ca_set_config(
        &self,
        conf: &CAConfig,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/connect/ca/configuration",
            Some(conf),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }
}
