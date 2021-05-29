extern crate chrono;
extern crate serde;
extern crate reqwest;
extern crate snafu;

#[macro_use]
extern crate derive_builder;

pub mod observation;
pub mod station;
pub mod windy_request;
pub mod windy_instance;