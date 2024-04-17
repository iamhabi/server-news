use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::news;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct News {
    pub id: i32,
    pub title: String,
    pub link: String,
}

#[derive(Insertable)]
#[diesel(table_name = news)]
pub struct NewNews<'a> {
    pub title: &'a str,
    pub link: &'a str,
}