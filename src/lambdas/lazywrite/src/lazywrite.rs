use diesel::dsl::sql_query;
use diesel::pg::PgConnection;
use diesel::sql_types::*;
use diesel::RunQueryDsl;
use failure::Error;

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

pub struct Controller<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> Controller<'a> {
    pub fn get_movies(&self) -> Result<String, Error> {
        let movies = sql_query("select * from movies").load::<Movie>(self.connection)?;
        Ok(format!(
            "List of movies: {}",
            movies  // Join the list of movie names with a ', '
                .iter()
                .map(|m| m.title.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
