extern crate base64;
extern crate chrono;
extern crate cis_client;
extern crate cis_profile;
extern crate config;
extern crate image;
extern crate reqwest;
extern crate serde_json;
extern crate sha2;
extern crate url;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod app;
mod loader;
mod name;
mod pictures;
mod resize;
mod settings;
