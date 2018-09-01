use diesel::pg::PgConnection;
use failure::Error;

use ::lazywrite;
use ::db;

pub fn handle(_path:String) -> Result<String,Error>{
    let controller = lazywrite::Controller{
        connection :  &db::CONNECTION.lock().unwrap() as &PgConnection
    };
    controller.get_movies()
}
