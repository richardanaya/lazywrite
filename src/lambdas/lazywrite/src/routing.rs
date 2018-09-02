use diesel::pg::PgConnection;
use failure::Error;

use db;
use lazywrite;
use serde_json;

pub fn handle(path: String) -> Result<serde_json::Value, Error> {
    let controller = lazywrite::Controller {
        connection: &db::CONNECTION.lock().unwrap() as &PgConnection,
    };
    Ok(controller.get_movies(path)?)
}
