extern crate rocket;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![justdoit::http::get_task, justdoit::http::get_tasks],
    )
}
