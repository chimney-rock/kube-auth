use openssl::ssl::{SslMethod, SslAcceptor, SslAcceptorBuilder, SslFiletype};
use actix_web::{App, HttpServer, web, middleware};
use failure::Fallible;
use std::io;

use crate::db::Database;
use crate::settings::Settings;

mod api;
mod errors;

/// HTTP Server object.
pub struct Server {
  pub sys: actix_rt::SystemRunner
}

impl Server {
  /// Creates a new HTTP server using settings.
  /// 
  /// # Arguments
  /// * `settings` - Settings to use.
  pub fn from_settings(settings: &Settings) -> Fallible<Server> {
    let sys = actix_rt::System::new("heimdallr");

    // Initialize the database connection
    let database = Database::from_settings(&settings)?;
    
    let server = HttpServer::new(move || {
      App::new()
        .data(database.clone())
        .wrap(middleware::Logger::default())
        .service(
          web::scope("/api")
            .service(web::resource("/healthz").to_async(api::healthz))
            .service(
              web::resource("/authenticate")
                .route(web::post().to_async(api::authenticate))
            )
        )
    });

    if settings.inbound_listener.tls.enabled {
      server.bind_ssl(
        &settings.inbound_listener.address,
        Self::build_tls(&settings.inbound_listener.tls.private_key, &settings.inbound_listener.tls.cert)?
      )?.start();
    }
    else {
      server.bind(&settings.inbound_listener.address)?.start();
    }

    Ok(Server{ sys })
  }

  /// Starts the HTTP server.
  pub fn start(self) -> io::Result<()> {
    self.sys.run()
  }

  /// Creates an SSL Acceptor object.
  /// 
  /// # Arguments
  /// * `tls` - TLS configuration settings.
  fn build_tls(private_key: &str, cert: &str) -> Fallible<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(private_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert)?;
    Ok(builder)
  }
}
