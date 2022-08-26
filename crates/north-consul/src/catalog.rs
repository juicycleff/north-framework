use async_trait::async_trait;
use std::collections::HashMap;

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Weights {
    pub Passing: u32,
    pub Warning: u32,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Node {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub Meta: HashMap<String, String>,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogService {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub NodeMeta: HashMap<String, String>,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceAddress: String,
    pub ServiceTags: Vec<String>,
    pub ServiceMeta: HashMap<String, String>,
    pub ServicePort: u32,
    pub ServiceWeights: Weights,
    pub ServiceEnableTagOverride: bool,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogNode {
    pub Node: Option<Node>,
    pub Services: HashMap<String, AgentService>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct CatalogRegistration {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub NodeMeta: HashMap<String, String>,
    pub Datacenter: String,
    pub Service: Option<AgentService>,
    pub Check: Option<AgentCheck>,
    pub SkipNodeUpdate: bool,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogDeregistration {
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub ServiceID: String,
    pub CheckID: String,
}

#[async_trait]
pub trait Catalog {
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)>;
    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
}

#[async_trait]
impl Catalog for Client {
    /// https://www.consul.io/api/catalog.html#register-entity
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/session/create",
            Some(reg),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#deregister-entity
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/catalog/deregister",
            Some(dereg),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#list-datacenters
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#list-nodes
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)> {
        get("/v1/catalog/nodes", &self.config, HashMap::new(), q).await
    }

    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q).await
    }
}
