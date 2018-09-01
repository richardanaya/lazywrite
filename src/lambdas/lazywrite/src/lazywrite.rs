use diesel::dsl::sql_query;
use diesel::pg::PgConnection;
use diesel::sql_types::*;
use diesel::RunQueryDsl;
use failure::Error;
use serde_json;

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

pub struct Controller<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> Controller<'a> {
    pub fn get_movies(&self) -> Result<serde_json::Value, Error> {
        let movies = sql_query("select * from movies").load::<Movie>(self.connection)?;
        Ok(json!({
            "movies": movies.iter().map(|m| m.title.to_string()).collect::<Vec<_>>()
        }))
    }
}
