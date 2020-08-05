use std::env;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpResponse, HttpServer, Responder};

pub mod person;
pub mod database;

static SERVER_BINDING: &str = "0.0.0.0:4000";
static MONGODB_URI: &str = "mongodb://root:tutorial@127.0.0.1:27017";
static MONGODB_DATABASE: &str = "tutorials";

async fn healthcheck() -> impl Responder {
  HttpResponse::Ok().body("OK!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let mongo_uri = match env::var("MONGO_URI") {
    Ok(uri) => uri,
    Err(_) => MONGODB_URI.to_owned(),
  };

  let mongo_database = match env::var("MONGO_DATABASE") {
    Ok(db_name) => db_name,
    Err(_) => MONGODB_DATABASE.to_owned(),
  };

  let db = match database::connect(mongo_uri.as_str(), mongo_database.as_str()).await {
    Err(e) => {
      panic!(e);
    },
    Ok(mongo_db) => mongo_db,
  };

  let server = HttpServer::new(move || {
    App::new()
      .data(db.clone())
      .wrap(middleware::Logger::new("%U - %s - %b bytes - %D ms"))
      .wrap(
        Cors::new()
          .allowed_origin("https://my-domain.com")
          .allowed_methods(vec!["GET", "PATCH", "POST"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
          .max_age(500)
          .finish()
      )
      .service(
        web::resource("/healthcheck")
          .route(web::get().to(healthcheck))
      )
      .service(
        web::scope("/api")
          .configure(person::configure)
      )
  });

  let result = server.bind(SERVER_BINDING);

  match result {
    Ok(server_instance) => {
      println!("Server listening on {}", SERVER_BINDING);
      server_instance.run().await
    },
    Err(e) => {
      println!("Cannot bind server on {} {}", SERVER_BINDING, e);
      Err(e)
    }
  }
}
