#[forbid(unsafe_code)]

#[macro_use]
extern crate serde;
extern crate core;
extern crate serde_json;
extern crate tuple;

mod macros;

mod prelude;
mod north;
pub mod contracts;
pub mod web;
mod router;
mod service;
mod error;
#[cfg(feature = "api-native")]
mod server;

mod addr;
mod helper;
mod utils;

pub use self::utils::server_utils::print_server_info;
pub use self::utils::server_utils::NorthResult;

pub use {
    self::error::{Error, ErrorResponse},
    self::service::{NorthServiceOptions},
    self::contracts::{NorthServiceBuilderTrait},
    north_common::state::NorthStateData,
    self::north::{North, new_service, power},
};
