use std::env;

use database::models::User;
use diesel::{PgConnection, Connection, RunQueryDsl};
extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Keep the databse info in mind to drop them later
struct TestContext {
    base_url: String,
    db_name: String,
}

impl TestContext {
    fn new(base_url: &str, db_name: &str) -> Self {

        let mut conn =
            PgConnection::establish(base_url).expect("Cannot connect to postgres database.");
        
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        // Create a new database for the test
        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
        query
            .execute(&mut conn)
            .expect(format!("Could not create database {}", db_name).as_str());

            Self {
                    base_url: base_url.to_string(),
                    db_name: db_name.to_string(),
            }
    }        
}


impl Drop for TestContext {

    fn drop(&mut self) {
        let postgres_url = format!("{}/postgres", self.base_url);
        let mut conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '{}';",
            self.db_name
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&mut conn)
            .unwrap();


        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query
            .execute(&mut conn)
            .expect(&format!("Couldn't drop database {}", self.db_name));
    }
}

fn setup_database() -> TestContext {
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = "mentor_db_test";

    return TestContext::new(&database_url, db_name);
}

#[test]
fn get_user_with_username_test() {
    let _ctx = setup_database();

    let mut conn = PgConnection::establish(&format!("postgres://postgres:example@127.0.0.1/sometest1"))
        .unwrap();

    // Now do your test.
    diesel::sql_query(
        "INSERT INTO users (email, username) VALUES ('MAIL', 'NAME')",
    )
    .execute(&mut conn)
    .unwrap();
    let u = User::get_with_username(&mut conn, "UserName")
        .unwrap();

    assert_eq!(u.email, "MAIL".to_string());
}