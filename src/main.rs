#[macro_use]
extern crate clap;

extern crate actix_web;

use failure::Fallible;
use clap::{App, Arg};

mod db;
mod logging;
mod server;
mod settings;

use settings::Settings;
use server::Server;

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
