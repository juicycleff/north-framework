pub(crate) use north_consul::catalog::CatalogRegistration;
pub(crate) use std::collections::HashMap;

pub(crate) use north_common::discovery::discovery_options::DiscoveryOptions;
pub(crate) use north_common::registry::registration::Registration;
pub(crate) use north_common::registry::service_instance::{IService, ServiceInstance};
pub(crate) use north_common::registry::service_instance_state::ServiceInstanceState;

pub(crate) use crate::clients::consul::registry::consul_registration::{
    ConsulDiscoveryOption, ConsulDiscoveryOptions, ConsulRegistration,
};

pub(crate) use nanoid::nanoid;
pub(crate) use north_common::discovery::heartbeat::HeartbeatOptions;
pub(crate) use north_common::registry::service_registry_builder::RegistrationBuilder;
pub(crate) use north_consul::agent::{AgentCheck, AgentService};

pub(crate) use crate::clients::base_client::ReactiveClient;
pub(crate) use crate::clients::consul::consul_client::ConsulReactiveClient;
pub(crate) use async_trait::async_trait;
pub(crate) use north_consul::catalog::{Catalog, CatalogDeregistration};
pub(crate) use north_consul::health::{Health, ServiceEntry};

pub(crate) use north_common::discovery::discovery_client::DiscoveryClient;
pub(crate) use north_common::error::Error;
pub(crate) use north_common::registry::default_service_instance::DefaultServiceInstance;
pub(crate) use north_common::registry::service_instance::ServiceInstanceOptions;

pub(crate) use crate::clients::consul::registry::consul_registration_builder::ConsulRegistrationBuilder;

pub(crate) use log::info;
pub(crate) use north_common::registry::service_registry::ServiceRegistry;
pub(crate) use north_consul::Config as ConsulConfig;

pub(crate) use north_consul::Client as ConsulClient;
pub(crate) use std::mem::MaybeUninit;
pub(crate) use std::sync::{Mutex, Once};
