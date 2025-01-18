extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    let _connection = PgConnection::establish("postgresql://localhost/rust_postgres").unwrap();
    println!("Connection to the database established!");
}
