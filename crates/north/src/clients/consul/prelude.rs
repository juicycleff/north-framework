pub(crate) use consul::catalog::CatalogRegistration;
pub(crate) use std::collections::HashMap;

pub(crate) use crate::discovery::discovery_options::DiscoveryOptions;
pub(crate) use crate::registry::registration::Registration;
pub(crate) use crate::registry::service_instance::{IService, ServiceInstance};
pub(crate) use crate::registry::service_instance_state::ServiceInstanceState;

pub(crate) use crate::clients::consul::registry::consul_registration::{
    ConsulDiscoveryOption, ConsulDiscoveryOptions, ConsulRegistration,
};

pub(crate) use crate::discovery::heartbeat::HeartbeatOptions;
pub(crate) use crate::registry::service_registry_builder::RegistrationBuilder;
pub(crate) use consul::agent::{AgentCheck, AgentService};
pub(crate) use nanoid::nanoid;

pub(crate) use crate::clients::base_client::ReactiveClient;
pub(crate) use crate::clients::consul::consul_client::ConsulReactiveClient;
pub(crate) use async_trait::async_trait;
pub(crate) use consul::catalog::{Catalog, CatalogDeregistration};
pub(crate) use consul::health::{Health, ServiceEntry};

pub(crate) use crate::discovery::discovery_client::DiscoveryClient;
pub(crate) use crate::registry::default_service_instance::DefaultServiceInstance;
pub(crate) use crate::registry::service_instance::ServiceInstanceOptions;
pub(crate) use crate::utils::server_utils::NorthResult;

pub(crate) use crate::clients::consul::registry::consul_registration_builder::ConsulRegistrationBuilder;

pub(crate) use crate::registry::service_registry::ServiceRegistry;
pub(crate) use consul::Config as ConsulConfig;
pub(crate) use log::info;

pub(crate) use consul::Client as ConsulClient;
pub(crate) use std::mem::MaybeUninit;
pub(crate) use std::sync::{Mutex, Once};
