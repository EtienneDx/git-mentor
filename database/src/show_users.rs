use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for post in results {
        println!("{}", post.username);
        println!("-----------\n");
        println!("{}", post.email);
    }
}