#[macro_use]
extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::fs::{ FileServer, relative };
use rocket_dyn_templates::Template;

use std::collections::HashMap;

use sql;
use scrap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub news: Vec<sql::models::News>,
    pub page_count: i64
}

#[get("/?<count>&<page>")]
async fn home(count: Option<i64>, page: Option<i64>) -> Template {
    let count = count.unwrap_or(100);
    let page = page.unwrap_or(1);

    let offset = count * (page - 1);

    let news = sql::get_news(count, offset);
    let page_count = sql::get_count() / count;

    let mut map = HashMap::new();

    let data = Data {
        news,
        page_count
    };

    map.insert("data", data);

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
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}