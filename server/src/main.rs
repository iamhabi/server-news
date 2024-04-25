#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket_dyn_templates::Template;

use std::collections::HashMap;

use sql;
use scrap;

#[get("/?<count>&<offset>")]
async fn home(count: Option<i64>, offset: Option<i64>) -> Template {
    let count = count.unwrap_or(100);
    let offset = offset.unwrap_or(0);

    let news = sql::get_news(count, offset);

    let mut map = HashMap::new();
    map.insert("news", news);

    Template::render("home", map)
}

#[get("/json?<count>&<offset>")]
pub fn get_news_as_json(count: Option<i64>, offset: Option<i64>) -> Json<Vec<sql::models::News>> {
    let count = count.unwrap_or(100);
    let offset = offset.unwrap_or(0);

    let news = sql::get_news(count, offset);

    Json(news)
}

#[get("/count")]
async fn get_count() -> Json<i64> {
    Json(sql::get_count())
}

#[rocket::launch]
pub fn rocket() -> _ {
    scrap::loop_scrap();

    rocket::build()
        .mount("/", routes![home, get_news_as_json, get_count])
        .attach(Template::fairing())
}