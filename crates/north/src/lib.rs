#[macro_use]
extern crate serde;
extern crate core;
extern crate serde_json;

mod server;

pub mod helper;
pub mod utils;
pub use self::utils::server_utils::NorthResult;

pub use {
    self::server::error::{Error, ErrorResponse},
    self::server::service::{NorthServiceOptions, NorthServiceBuilderTrait},
    self::server::north::{North, new_service, power},
};
