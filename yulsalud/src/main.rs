#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "¡Hola mundo desde Rocket y Rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
