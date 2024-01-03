#[forbid(unsafe_code)]
#[macro_use]
extern crate serde;
extern crate core;
extern crate serde_json;
extern crate tuple;

mod macros;

pub mod contracts;
mod error;
mod north;
mod prelude;
mod router;
#[cfg(feature = "api-native")]
mod server;
mod service;
pub mod web;

mod addr;
pub mod helper;
mod utils;

pub use self::utils::server_utils::print_server_info;
pub use self::utils::server_utils::NorthResult;

pub use {
    self::contracts::NorthServiceBuilderTrait,
    self::error::{Error, ErrorResponse},
    self::north::{new_service, power, North},
    self::service::NorthServiceOptions,
    north_common::state::NorthStateData,
};
