use diesel::prelude::*;
use rust_by_example::*;
use std::env::args;

fn main() {
    use schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(connection)
        .unwrap();

    let post: models::Post = posts
        .find(id)
        .first(connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
