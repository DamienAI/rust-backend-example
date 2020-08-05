use serde::{Deserialize, Serialize};

use mongodb::{bson};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
  #[serde(rename = "_id")]
  pub id: bson::oid::ObjectId,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditablePerson {
  pub name: String,
}

