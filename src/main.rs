#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::Html;

mod phone_param;
use phone_param::PhoneParam;

#[get("/<phone_number>")]
fn index(phone_number: PhoneParam) -> Html<String> {
    Html(format!("<meta http-equiv=\"refresh\" content=\"0; URL='tel:+1#{}'\" />", phone_number.phone_number))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
