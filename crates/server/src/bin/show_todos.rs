use diesel::prelude::*;
use server::models::*;
use server::*;

fn main() {
    use server::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    println!("Connecting to database...");
    let results = todo
        .filter(done.eq(true))
        .limit(5)
        .load::<Todo>(connection)
        .expect("Error loading posts");

    println!("Displaying {} todos", results.len());
    for r in results {
        println!("{}", r.title);
    }
}
