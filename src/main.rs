use rocket::{Build, Rocket};

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", rocket::routes![index])
}
