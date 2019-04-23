extern crate diesel_try;
extern crate diesel;

use self::diesel_try::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_try::schema::posts::dsl::*;

    let connection = establish_connectoin();
    let result = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", result.len());
    for post in result {
        println!("{}", post.title);
        println!("--------\n");
        println!("{} ", post.body);
    }
}