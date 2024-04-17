use self::models::*;
use diesel::prelude::*;
use sql::*;

fn main() {
    use self::schema::news::dsl::*;

    let connection = &mut establish_connection();
    
    let results: Vec<News> = news
        .select(News::as_select())
        .load(connection)
        .expect("Error loading news");

    println!("Displaying {} news", results.len());

    for n in results {
        println!("{}", n.title);
    }
}