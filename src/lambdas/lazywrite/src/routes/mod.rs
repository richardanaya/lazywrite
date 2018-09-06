use http::Method;
use route::Route;
use routes;
pub mod movies;

lazy_static! {
    pub static ref ROUTES: Vec<Route<'static>> = {
        vec![Route::new(
            Method::GET,
            "/movies".to_string(),
            &routes::movies::get_movies,
        )]
    };
}
