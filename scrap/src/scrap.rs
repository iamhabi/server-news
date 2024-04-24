use serde::{Serialize, Deserialize};

use reqwest;
use soup::prelude::*;

use tokio::runtime::Runtime;

use chrono::{DateTime, Utc};

use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

use sql;

#[derive(Debug, Serialize, Deserialize)]
struct ScrapInfo {
    parent_url: String,
    url: String,
    tag: String,
    attr: (String, String),
}

pub fn scrap_news() {
    let scrap_infos = get_scrap_infos();

    for info in scrap_infos {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async move {
            get_news_from_url(info).await;
        });
    }

    print_scrap_time();
}

async fn get_news_from_url(info: ScrapInfo) {
    let response = reqwest::get(info.url).await.unwrap();
    let body = response.text().await.unwrap();
    let soup = Soup::new(&body);

    let first_attr = info.attr.0;
    let second_attr = info.attr.1;

    let tags = soup
        .tag(info.tag)
        .attr(first_attr, second_attr)
        .find_all()
        .enumerate();

    for (_, tag) in tags {
        let title = tag.text();

        let a = tag.tag("a")
            .find()
            .expect("Couldn't find link with 'a' attribute");

        let mut link = a.get("href")
            .expect("Couldn't find link with 'href' attribute");

        if !link.starts_with("http") {
            link = format!("{}{}", info.parent_url, link);
        }

        sql::insert_news(&title, &link);
    }
}

fn get_scrap_infos() -> Vec<ScrapInfo> {
    let mut news_info_json = File::open("news_info.json")
        .expect("Failed to open json file");

    let mut data = String::new();
    news_info_json.read_to_string(&mut data)
        .expect("Failed to parse json");

    serde_json::from_str(&data).unwrap()
}

fn print_scrap_time() {
    let now = SystemTime::now();

    let datetime = DateTime::<Utc>::from(now);

    print!("{}", datetime.format("%Y-%m-%d %H:%M:%S").to_string());
}
