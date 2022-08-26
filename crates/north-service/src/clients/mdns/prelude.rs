pub(crate) use std::collections::HashMap;

pub(crate) use north_common::discovery::discovery_options::DiscoveryOptions;
pub(crate) use north_common::registry::registration::Registration;
pub(crate) use north_common::registry::service_instance::{IService, ServiceInstance};
pub(crate) use north_common::registry::service_instance_state::ServiceInstanceState;

pub(crate) use north_common::discovery::heartbeat::HeartbeatOptions;
pub(crate) use north_common::registry::service_registry_builder::RegistrationBuilder;
pub(crate) use nanoid::nanoid;

pub(crate) use crate::clients::base_client::ReactiveClient;
pub(crate) use async_trait::async_trait;

pub(crate) use crate::clients::mdns::mdns_registration::*;
pub(crate) use crate::clients::mdns::mdns_registration_builder::*;
pub(crate) use north_common::registry::service_registry::ServiceRegistry;

pub(crate) use log::info;
pub(crate) use std::mem::MaybeUninit;
pub(crate) use std::sync::{Mutex, Once};
