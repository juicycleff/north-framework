use async_trait::async_trait;
use std::collections::HashMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::request::{delete, get, get_vec, put, put_body};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct KVPair {
    pub Key: String,
    pub CreateIndex: Option<u64>,
    pub ModifyIndex: Option<u64>,
    pub LockIndex: Option<u64>,
    pub Flags: Option<u64>,
    pub Value: String,
    pub Session: Option<String>,
}

#[allow(clippy::upper_case_acronyms)]
#[async_trait]
pub trait KV {
    async fn acquire(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    async fn delete(&self, _: &str, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    async fn get(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Option<KVPair>, QueryMeta)>;
    async fn list(&self, _: &str, _: Option<&QueryOptions>) -> Result<(Vec<KVPair>, QueryMeta)>;
    async fn put(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
    async fn release(&self, _: &KVPair, _: Option<&WriteOptions>) -> Result<(bool, WriteMeta)>;
}

#[async_trait]
impl KV for Client {
    async fn acquire(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("acquire"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put(&path, Some(&pair.Value), &self.config, params, o).await
        } else {
            Err(Error::from("Session flag is required to acquire lock"))
        }
    }

    async fn delete(&self, key: &str, options: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let path = format!("/v1/kv/{}", key);
        delete(&path, &self.config, HashMap::new(), options).await
    }

    async fn get(
        &self,
        key: &str,
        options: Option<&QueryOptions>,
    ) -> Result<(Option<KVPair>, QueryMeta)> {
        let path = format!("/v1/kv/{}", key);
        let x: Result<(Vec<KVPair>, QueryMeta)> =
            get(&path, &self.config, HashMap::new(), options).await;
        x.map(|r| (r.0.first().cloned(), r.1))
    }

    async fn list(
        &self,
        prefix: &str,
        o: Option<&QueryOptions>,
    ) -> Result<(Vec<KVPair>, QueryMeta)> {
        let mut params = HashMap::new();
        params.insert(String::from("recurse"), String::from(""));
        let path = format!("/v1/kv/{}", prefix);
        get_vec(&path, &self.config, params, o).await
    }

    async fn put(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        let path = format!("/v1/kv/{}", pair.Key);
        put_body(&path, Some(pair.Value.clone()), &self.config, params, o).await
    }

    async fn release(&self, pair: &KVPair, o: Option<&WriteOptions>) -> Result<(bool, WriteMeta)> {
        let mut params = HashMap::new();
        if let Some(i) = pair.Flags {
            if i != 0 {
                params.insert(String::from("flags"), i.to_string());
            }
        }
        if let Some(ref session) = pair.Session {
            params.insert(String::from("release"), session.to_owned());
            let path = format!("/v1/kv/{}", pair.Key);
            put(&path, Some(&pair.Value), &self.config, params, o).await
        } else {
            Err(Error::from("Session flag is required to release a lock"))
        }
    }
}
