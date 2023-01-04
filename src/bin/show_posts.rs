use diesel::prelude::*;
use rust_by_example::*;
use self::establish_connection;
use self::models::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(10)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}:{}", post.id, post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}