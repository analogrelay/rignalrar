#![feature(async_await, await_macro, futures_api)]

extern crate url;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

mod hub_connection;
mod error;

pub use self::hub_connection::HubConnection;
pub use self::error::Error;