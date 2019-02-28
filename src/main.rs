#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<phone_number>")]
fn index(phone_number: &RawStr) -> String {
    format!("<div class='poop'>Hello! Your number is: {}</div>", phone_number.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
