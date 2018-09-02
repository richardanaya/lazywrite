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

use failure::Error;
use lambda::event::apigw::ApiGatewayProxyRequest;
use lambda::Context;
use warp::Filter;

mod db;
mod lazywrite;
mod routing;

fn handle_request(e: ApiGatewayProxyRequest, _ctx: Context) -> Result<serde_json::Value, Error> {
    match routing::handle(e.path) {
        Ok(json) => Ok(json!({
              "statusCode": 200,
              "headers" : {
                  "Content-Type" : "text/json"
              },
              "body": json.to_string()
          })),
        Err(e) => Ok(json!({
              "statusCode": 500,
              "headers" : {
                  "Content-Type" : "text/json"
              },
              "body": format!("An unexpected error occurred: {}",e)
          })),
    }
}

fn start_local_server() {
    let api = path!("api")
        .and(warp::path::tail())
        .map(|tail: warp::path::Tail| {
            let path = format!("/{}", tail.as_str().to_owned());
            warp::reply::json(&routing::handle(path).unwrap())
        });
    let index = warp::index().and(warp::fs::file("../../../dist/website/index.html"));
    let static_files = warp::any().and(warp::fs::dir("../../../dist/website"));
    warp::serve(api.or(index).or(static_files)).run(([0, 0, 0, 0], 3030));
}

/// Start listening for AWS Lambda requests for API Gateway.
fn main() {
    if cfg!(feature = "local_development") {
        start_local_server();
    } else {
        lambda::start(move |e: ApiGatewayProxyRequest| {
            let ctx = Context::current();
            handle_request(e, ctx)
        });
    }
}
