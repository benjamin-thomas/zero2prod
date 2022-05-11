extern crate core;

pub mod config;

pub(crate) mod domain;

pub(crate) mod routes;
pub mod startup;
pub mod telemetry;

pub use startup::*;
