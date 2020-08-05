pub mod service;
pub mod model;

use actix_web::{web};
use actix_web::FromRequest;
use mime;

use crate::person::model::EditablePerson;
use crate::person::service::{create_person, get_person, json_error_handler};

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/persons")
    .app_data(web::Json::<EditablePerson>::configure(|cfg| { 
      cfg
        .limit(4096)
        .content_type(|mime| {
          // accept only application/json
          mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON
        })
        .error_handler(json_error_handler)
    }))
    .route("/{person_id}", web::get().to(get_person))
    .route("", web::post().to(create_person))
  );
}
