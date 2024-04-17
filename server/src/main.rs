#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::tokio::runtime::Runtime;

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

fn loop_scrap_news() {
    let delay_time = std::time::Duration::from_millis(1000 * 60 * 60);

    let runtime = Runtime::new().unwrap();

    runtime.block_on(async move {
        loop {
            scrap::scrap();

            std::thread::sleep(delay_time);
        }
    });
}

#[rocket::launch]
pub fn rocket() -> _ {
    // loop_scrap_news();

    rocket::build()
        .mount("/", routes![home])
        .attach(Template::fairing())
}