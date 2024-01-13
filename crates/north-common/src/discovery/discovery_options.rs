use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
#[derive(Default)]
pub enum DiscoveryHttpMethods {
    Post,
    #[default]
    Get,
    Delete,
    Patch,
    Options,
    Head,
    Put,
}


/*
#[derive(Default, Clone)]
pub struct ScriptDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    args: Vec<String>,
    timeout: Option<i64>,
    script: String,
}

#[derive(Default, Clone)]
pub struct HttpDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,

    body: Option<String>,
    header: HashMap<String, String>,
    http: String,
    skip_verify_tls: Option<bool>,
    method: DiscoveryHttpMethods,
}

#[derive(Default, Clone)]
pub struct TcpDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,
    tcp: String,
}

#[derive(Default, Clone)]
pub struct DockerDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,
    shell: String,
    args: Option<String>,
    docker_container_id: String,
}

#[derive(Default, Clone)]
pub struct GrpcDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,
    grpc: String,
    use_tls: bool,
}

#[derive(Default, Clone)]
pub struct TtlDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,
    ttl: Option<i64>,
}

#[derive(Default, Clone)]
pub struct AliasServiceDiscoveryOptions {
    service_id: String,
    service_name: String,
    interval: i64,
    fail_fast: bool,
    scheme: String,

    timeout: Option<i64>,
    ttl: Option<i64>,
    alias_service: String,
    alias_node: Option<String>,
}
*/

#[derive(Default, Clone, Debug)]
pub struct DiscoveryOptions {
    pub service_id: String,
    pub service_name: String,
    pub interval: i64,
    pub fail_fast: bool,
    pub scheme: String,

    pub timeout: Option<i64>,

    pub body: Option<String>,
    pub header: HashMap<String, String>,
    pub http: String,
    pub skip_verify_tls: Option<bool>,
    pub method: DiscoveryHttpMethods,
}

// pub union DiscoveryOptions {
//     http: ManuallyDrop<HttpDiscoveryOptions>,
//     alias: ManuallyDrop<AliasServiceDiscoveryOptions>,
//     grpc: ManuallyDrop<GrpcDiscoveryOptions>,
//     ttl: ManuallyDrop<TtlDiscoveryOptions>,
//     docker: ManuallyDrop<DockerDiscoveryOptions>,
//     script: ManuallyDrop<ScriptDiscoveryOptions>,
// }
