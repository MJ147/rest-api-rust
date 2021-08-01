#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/movies", routes![])
}
