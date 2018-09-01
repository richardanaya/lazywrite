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
      "body": routing::handle(e.path)?
    }))
}

fn start_local_server() {
    let hello = warp::any().map(|| routing::handle("/".to_owned()).unwrap());
    warp::serve(hello).run(([0, 0, 0, 0], 3030));
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
