use crate::clients::mdns::prelude::*;

#[derive(Default, Debug)]
pub struct MdnsRegistrationBuilder {
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
    _discovery_options: Option<MdnsDiscoveryOptions>,
}

impl MdnsRegistrationBuilder {
    pub fn new(host: Option<String>, port: Option<u32>) -> MdnsRegistrationBuilder {
        MdnsRegistrationBuilder {
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

impl RegistrationBuilder<MdnsRegistration, MdnsContext> for MdnsRegistrationBuilder {
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
        let cdo = options;
        self._discovery_options = Option::from(cdo);
        self
    }

    fn heartbeat_options(mut self, options: HeartbeatOptions) -> Self {
        self._heartbeat_options = options;
        self
    }

    fn build(mut self) -> MdnsRegistration {
        let svc_name = self._service_name.expect("[service name] is required");
        let host = self._host.expect("[service hostname] is required");
        let port = self._port.expect("[service port] is required");
        let dso = self
            ._discovery_options
            .expect("[discovery options] is required");
        let domain = self._domain.unwrap_or_else(|| "north".to_string());
        let version = self._version.unwrap_or_else(|| "latest".to_string());

        // let scheme = dso.scheme;
        let is_secure = true;

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

        let ctx = MdnsContext {
            service_name: svc_name.clone(),
            node_id: instance_id.clone(),
            service_id: svc_name,
            instance_id,
            host,
            status: "".to_string(),
            port,
            secure: is_secure,
            metadata: None,
            tags: None,
        };

        MdnsRegistration::new(ctx, dso)
    }
}
