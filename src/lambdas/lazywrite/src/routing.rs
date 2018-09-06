use failure::Error;
use http::Method;
use routes;
use serde_json;

pub fn route(method: Method, path: String) -> Result<serde_json::Value, Error> {
    for r in routes::ROUTES.iter() {
        match r.match_path(&method, &path) {
            Some(params) => return (r.executor)(params),
            None => (),
        }
    }
    return Err(format_err!("Unknown route."));
}
