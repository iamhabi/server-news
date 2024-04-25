use diesel::{associations::HasTable, QueryDsl};
use diesel::prelude::*;

use sql::*;

fn main() {
    use self::schema::news::dsl::*;

    let connection = &mut establish_connection();

    let count: i64 = news::table()
        .count()
        .get_result(connection)
        .expect("Error get size");

    println!("{}", count);
}