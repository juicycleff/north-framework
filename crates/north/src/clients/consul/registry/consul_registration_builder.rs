use crate::clients::consul::prelude::*;

#[derive(Default)]
pub struct ConsulRegistrationBuilder {
    _service_name: Option<String>,
    _port: Option<u32>,
    _host: Option<String>,
    _status: Option<String>,
    _version: Option<String>,
    _tags: Option<Vec<String>>,
    _domain: Option<String>,
    _meta: Option<HashMap<String, String>>,
    _instance_id: Option<String>,
    _heartbeat_options: HeartbeatOptions,
    _discovery_options: Option<ConsulDiscoveryOptions>,
}

impl ConsulRegistrationBuilder {
    pub fn new(host: Option<String>, port: Option<u32>) -> ConsulRegistrationBuilder {
        ConsulRegistrationBuilder {
            _service_name: None,
            _port: port,
            _host: host,
            _status: None,
            _version: Some("latest".into()),
            _tags: None,
            _domain: Some("north".into()),
            _meta: None,
            _instance_id: None,
            _heartbeat_options: HeartbeatOptions::default(),
            _discovery_options: None,
        }
    }
}

impl RegistrationBuilder<ConsulRegistration, CatalogRegistration> for ConsulRegistrationBuilder {
    fn service_name(mut self, name: String) -> Self {
        self._service_name = Option::from(name);
        self
    }

    fn tags(mut self, tags: Vec<String>) -> Self {
        self._tags = Option::from(tags);
        self
    }

    fn instance_id(mut self, id: String) -> Self {
        self._instance_id = Option::from(id);
        self
    }

    fn host(mut self, host: String) -> Self {
        self._host = Option::from(host);
        self
    }

    fn port(mut self, port: u32) -> Self {
        self._port = Option::from(port);
        self
    }

    fn version(mut self, version: String) -> Self {
        self._version = Option::from(version);
        self
    }

    fn status(mut self, status: String) -> Self {
        self._status = Option::from(status);
        self
    }

    fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self._meta = Option::from(metadata);
        self
    }

    fn domain(mut self, domain: String) -> Self {
        self._domain = Option::from(domain);
        self
    }

    fn discovery_options(mut self, options: DiscoveryOptions) -> Self {
        let cdo = ConsulDiscoveryOption {
            discovery_options: options,
            scheme: "".to_string(),
            fail_fast: None,
            notes: None,
            script: None,
            deregister_critical_service_after: None,
        };

        self._discovery_options = Option::from(cdo);
        self
    }

    fn heartbeat_options(mut self, options: HeartbeatOptions) -> Self {
        self._heartbeat_options = options;
        self
    }

    fn build(mut self) -> ConsulRegistration {
        let svc_name = self._service_name.expect("[service name] is required");
        let host = self._host.expect("[service hostname] is required");
        let port = self._port.expect("[service port] is required");
        let dso = self
            ._discovery_options
            .expect("[discovery options] is required");
        let domain = self._domain.unwrap_or_else(|| "north".to_string());
        let version = self._version.unwrap_or_else(|| "latest".to_string());

        // let scheme = dso.scheme;
        let is_secure = dso.scheme == "https";

        let mut tags = self._tags.unwrap_or_default();
        tags.push("service".to_string());
        tags.push("north".to_string());
        tags.push(version.clone());

        let mut meta = self._meta.unwrap_or_default();
        meta.insert("domain".into(), domain);
        meta.insert("version".into(), version);
        meta.insert("secure".into(), is_secure.to_string());

        let instance_id = match self._instance_id {
            None => {
                format!("{sn}-{id}", sn = svc_name, id = nanoid!())
            }
            Some(iid) => {
                format!("{sn}-{id}", sn = iid, id = nanoid!())
            }
        };

        if self._heartbeat_options.enabled && self._heartbeat_options.ttl_in_seconds != None {
            self._heartbeat_options.ttl_in_seconds = Some(120);
        };

        let serv = CatalogRegistration {
            ID: instance_id.clone(),
            Node: "".to_string(),
            Address: host.clone(),
            TaggedAddresses: Default::default(),
            NodeMeta: meta,
            Datacenter: "".to_string(),
            Service: Some(AgentService {
                ID: svc_name.clone(),
                Service: "".to_string(),
                Tags: Some(tags),
                Port: port as u16,
                Address: host,
                EnableTagOverride: false,
                CreateIndex: 0,
                ModifyIndex: 0,
            }),
            Check: Option::from(AgentCheck {
                Node: "".to_string(),
                CheckID: "".to_string(),
                Name: svc_name.clone() + "Status",
                Status: "".to_string(),
                Notes: "".to_string(),
                Output: "".to_string(),
                ServiceID: instance_id,
                ServiceName: svc_name,
            }),
            SkipNodeUpdate: false,
        };

        ConsulRegistration::new(serv, dso)
    }
}
