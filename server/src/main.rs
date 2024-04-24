#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

use std::collections::HashMap;

use sql;
use scrap;

#[get("/?<limit>&<offset>")]
async fn home(limit: Option<i64>, offset: Option<i64>) -> Template {
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);

    let news = sql::get_news(limit, offset);

    let mut map = HashMap::new();
    map.insert("news", news);

    Template::render("home", map)
}

#[get("/json?<limit>&<offset>")]
pub fn get_news_as_json(limit: Option<i64>, offset: Option<i64>) -> Json<Vec<sql::models::News>> {
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);

    let news = sql::get_news(limit, offset);

    Json(news)
}

#[rocket::launch]
pub fn rocket() -> _ {
    scrap::loop_scrap();

    rocket::build()
        .mount("/", routes![home, get_news_as_json])
        .attach(Template::fairing())
}