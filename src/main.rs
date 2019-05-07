#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

// #[macro_use]
// extern crate validator_derive;

use failure::Fallible;
use clap::{App, Arg};

mod db;
mod models;
mod server;
mod logging;
mod settings;
mod kubernetes;

use settings::Settings;
use server::Server;

use diesel_migrations::embed_migrations;

embed_migrations!("./migrations");

fn main() -> Fallible<()> {
  logging::init()?;

  let arguments = App::new("Heimdallr")
    .about("API Authentication Service")
    .version(crate_version!())
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true)
    ).get_matches();

  // Figure out what config file to load
  let cwd = ::std::env::current_dir()?;
  let default_config = format!("{}/config.yaml", cwd.display());
  let config_file    = arguments.value_of("config").unwrap_or(&default_config);

  let settings = Settings::new(config_file)?;
  let server   = Server::from_settings(&settings)?;
  server.start()?;
  Ok(())
}
