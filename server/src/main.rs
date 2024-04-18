#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use std::collections::HashMap;

use sql;
use scrap;

#[get("/")]
async fn home() -> Template {
    let news = sql::get_news();

    let mut map = HashMap::new();
    map.insert("news", news);

    Template::render("home", map)
}

#[rocket::launch]
pub fn rocket() -> _ {
    scrap::loop_scrap();

    rocket::build()
        .mount("/", routes![home])
        .attach(Template::fairing())
}