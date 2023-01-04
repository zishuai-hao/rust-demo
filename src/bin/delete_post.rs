use diesel::prelude::*;
use rust_by_example::*;
use std::env::args;
use std::fmt::format;

fn main() {
    let conn = &mut establish_connection();
    use self::schema::posts::dsl::*;
    let num_deleted =
        diesel::delete(
            posts.filter(
                        title.like(
                            format!("%{}%", args().nth(1).expect("Expected a target to match against")))))
            .execute(conn).expect("Error deleting posts");
    println!("Deleted {} posts", num_deleted);
}