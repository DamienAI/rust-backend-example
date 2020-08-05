use mongodb::{bson, Client, options::ClientOptions, Database};

pub async fn connect(uri: &str, database: &str) -> Result<Database, String> {
  let mut client_options = match ClientOptions::parse(uri).await {
    Err(e) => return Err(format!("Error parsing mongo uri '{}': '{}'", uri, e)),
    Ok(opts) => opts,
  };
  client_options.app_name = Some("Rust backend".to_string());
  client_options.connect_timeout = Some(std::time::Duration::from_secs(2));

  println!("Connecting to mondodb using uri: {}", uri);
  let client = match Client::with_options(client_options) {
    Err(e) => return Err(format!("Cannot connect to mongo uri '{}': '{}'", uri, e)),
    Ok(mongo_client) => mongo_client,
  };

  match client
    .database("admin")
    .run_command(bson::doc! {"ping": 1}, None).await {
      Err(e) => return Err(format!("Cannot connect to mongo uri '{}': '{}'", uri, e)),
      Ok(_) => {},
  };
  println!("Connected successfully.");


  Ok(client.database(database))
}