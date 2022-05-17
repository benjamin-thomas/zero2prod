extern crate core;

pub mod config;
pub mod domain;
pub mod email_client;

pub mod background_jobs;
pub(crate) mod routes;

pub mod startup;
pub mod telemetry;

pub use startup::*;
