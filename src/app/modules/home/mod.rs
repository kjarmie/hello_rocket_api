use rocket::get;
use rocket::routes;

/* Route Handlers */
#[get("/home/index")]
fn index() -> String {
    format!("/home/index")
}

/* Export the routes */
pub fn get_routes() -> Vec<rocket::Route> {
    routes![index]
}
