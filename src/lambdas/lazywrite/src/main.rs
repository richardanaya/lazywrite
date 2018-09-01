extern crate aws_lambda as lambda;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate warp;

use warp::Filter;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use failure::Error;
use lambda::event::apigw::ApiGatewayProxyRequest;
use lambda::Context;

mod db;

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

/// Handle API Gateway request and return API Gateway response.
fn handle_request(e: ApiGatewayProxyRequest, _ctx: Context) -> Result<serde_json::Value, Error> {
    let connection = &db::CONNECTION.lock().unwrap() as &PgConnection;
    let movies = diesel::sql_query("select * from movies").load::<Movie>(connection)?;

    Ok(json!({
      "statusCode": 200,
      "body": format!(
          "List of movies for url path {}: {}",
          e.path, // The path of the current url (e.g. /index.html )
          movies  // Join the list of movie names with a ', '
              .iter()
              .map(|m| m.title.to_string())
              .collect::<Vec<_>>()
              .join(", "))
          }))
}

fn start_local_server(){
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String)
        .map(|name| {
            format!("hey")
        });

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030));
}

/// Start listening for AWS Lambda requests for API Gateway.
fn main() {
    if cfg!(feature="local_development") {
        start_local_server();
    } else {
        lambda::start(move |e: ApiGatewayProxyRequest| {
            let ctx = Context::current();
            handle_request(e, ctx)
        });
    }
}
