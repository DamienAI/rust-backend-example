use mongodb::{bson, Collection, Database};
use actix_web::{error, web, HttpRequest, HttpResponse, Responder};

use crate::person::model::{EditablePerson, Person};

// For json decoding errors
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
  let detail = err.to_string();
  let resp = match &err {
    error::JsonPayloadError::ContentType => {
          HttpResponse::UnsupportedMediaType().body(detail)
      }
      error::JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
          HttpResponse::UnprocessableEntity().body(detail)
      }
      _ => HttpResponse::BadRequest().body(detail),
  };
  error::InternalError::from_response(err, resp).into()
}

pub async fn insert_document(collection: Collection, doc: bson::Document) -> Result<bson::oid::ObjectId, String> {
  match collection.insert_one(doc, None).await {
    Ok(inserted) => match bson::from_bson(inserted.inserted_id) {
      Ok(res) => Ok(res),
      Err(_) => Err("Cannot get inserted ObjectId".into()),
    },
    Err(err) => Err(format!("Error inserting: {}", err)),
  }
}

pub async fn find_one_persion_by_id(db: &Database, id: bson::oid::ObjectId) -> Result<Option<Person>, String> {
  match db.collection("Person").find_one(Some(bson::doc! {"_id": id}), None).await {
    Ok(mongo_result) => match mongo_result {
      Some(document) => match bson::from_bson(bson::Bson::Document(document)) {
        Ok(person) => Ok(Some(person)),
        Err(_) => Err("Error reversing bson object".into()),
      },
      None => Ok(None),
    },
    Err(e) => Err(format!("Error, cannot get document: {}", e)),
  }
}


pub async fn insert_person(db: &Database, person: &EditablePerson) -> Result<bson::oid::ObjectId, String> {
  match bson::to_bson(person) {
    Ok(bson_object) => match bson_object {
      bson::Bson::Document(bson_doc) => insert_document(db.collection("Person"), bson_doc).await,
      _ => Err("Cannot create the bson document".into()),
    },
    Err(e) => Err(format!("Cannot create bson object: {}", e)),
  }
}

pub async fn get_person(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
  let object_id = match bson::oid::ObjectId::with_string(id.into_inner().as_str()) {
    Ok(result) => result,
    Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
  };

  match find_one_persion_by_id(&db.into_inner(), object_id).await {
    Ok(result) => match result {
      Some(value) => HttpResponse::Ok().json(value),
      None => HttpResponse::NotFound().body("person not found"),
    },
    Err(e) => HttpResponse::InternalServerError().body(format!("Error finding object: {}", e)),
  }
}

pub async fn create_person(db: web::Data<Database>, person: web::Json<EditablePerson>) -> impl Responder {
  match insert_person(&db.into_inner(), &person.into_inner()).await {
    Ok(obj_id) => HttpResponse::Ok().json(obj_id),
    Err(e) => HttpResponse::InternalServerError().body(format!("Error inserting object: {}", e)),
  }
}