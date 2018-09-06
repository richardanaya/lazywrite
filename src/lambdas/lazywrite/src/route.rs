use failure::Error;
use http::Method;
use serde_json;
use std::collections::HashMap;
use std::marker::Sync;

type Executor = Fn(HashMap<String, String>) -> Result<serde_json::Value, Error> + Sync;

pub struct Route<'a> {
    method: Method,
    path: String,
    pub executor: &'a Executor,
}

impl<'a> Route<'a> {
    pub fn new(method: Method, path: String, executor: &Executor) -> Route {
        Route {
            method: method,
            path: path,
            executor: executor,
        }
    }

    pub fn match_path(&self, _method: &Method, _path: &str) -> Option<HashMap<String, String>> {
        Some(HashMap::new())
    }
}
