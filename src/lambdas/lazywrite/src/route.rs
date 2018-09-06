use failure::Error;
use http::Method;
use serde_json;
use std::collections::HashMap;
use std::marker::Sync;

type Executor = Fn(HashMap<String, String>) -> Result<serde_json::Value, Error> + Sync;

pub struct Route<'a> {
    method: Method,
    path: &'a str,
    pub executor: &'a Executor,
}

impl<'a> Route<'a> {
    pub fn new(method: Method, path: &'a str, executor: &'a Executor) -> Route<'a> {
        Route {
            method: method,
            path: path,
            executor: executor,
        }
    }

    pub fn match_path(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if self.method != method {
            return None;
        }
        if self.path != path && self.path != "*" {
            return None;
        }
        Some(HashMap::new())
    }
}
