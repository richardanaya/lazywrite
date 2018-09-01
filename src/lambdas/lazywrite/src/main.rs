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
    Ok(json!({
      "statusCode": 200,
      "headers" : {
          "Content-Type" : "text/json"
      },
      "body": routing::handle(e.path)?.to_string()
    }))
}

fn start_local_server() {
    let api = path!("api")
        .map(||{
             warp::reply::json(&routing::handle("/blah".to_owned()).unwrap())
        });
    let index = warp::index()
        .and(warp::fs::file("../../../dist/website/index.html"));
    let static_files = warp::any()
    .and(warp::fs::dir("../../../dist/website"));
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
