uniffi::setup_scaffolding!();

pub(crate) mod http;

mod constants;
mod models;
mod session;

pub mod client;

pub use client::Client;
pub use http::errors::HttpError;
