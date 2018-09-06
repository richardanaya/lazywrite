use db;
use diesel::pg::PgConnection;
use failure::Error;
use lazywrite;
use serde_json;
use std::collections::HashMap;

pub fn get_movies(_params: HashMap<String, String>) -> Result<serde_json::Value, Error> {
    let controller = lazywrite::Controller {
        connection: &db::CONNECTION.lock().unwrap() as &PgConnection,
    };
    Ok(controller.get_movies("/blah".to_owned())?)
}
