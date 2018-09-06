use db;
use diesel::pg::PgConnection;
use failure::Error;
use http::Method;
use lazywrite;
use route::Route;
use serde_json;
use std::collections::HashMap;

lazy_static! {
    pub static ref ROUTES: Vec<Route<'static>> = {
        vec![
            Route::new(Method::GET, "/movies", &get_movies),
            Route::new(Method::GET, "*", &get_movies),
        ]
    };
}

pub fn get_movies(_params: HashMap<String, String>) -> Result<serde_json::Value, Error> {
    let controller = lazywrite::Controller {
        connection: &db::CONNECTION.lock().unwrap() as &PgConnection,
    };
    Ok(controller.get_movies("/blah".to_owned())?)
}
