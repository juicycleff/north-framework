#[cfg(feature = "registry-consul")]
pub mod consul;

#[cfg(feature = "registry-local")]
pub mod mdns;

pub mod base_client;
