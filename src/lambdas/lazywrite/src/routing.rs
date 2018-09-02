use diesel::pg::PgConnection;
use failure::Error;

use db;
use http::Method;
use lazywrite;
use serde_json;

pub fn route(method: Method, path: String) -> Result<serde_json::Value, Error> {
    let controller = lazywrite::Controller {
        connection: &db::CONNECTION.lock().unwrap() as &PgConnection,
    };
    if method == Method::GET {
        return Ok(controller.get_movies(path)?);
    }
    return Err(format_err!("Unknown route."))
}
