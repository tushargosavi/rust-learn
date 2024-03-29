extern crate diesel_try;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_try::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_try::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");

    let mut connection = establish_connectoin();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&mut connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}