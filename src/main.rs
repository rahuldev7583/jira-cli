use crate::db::*;

mod db;
mod model;
fn main() {
    println!("Welcome to JIRA!");

    let db = JSONFileDatabase {
        file_path: "./data/db.son".to_owned(),
    };
    let result = db.read_db();

    println!("result: {:?}", result);
    assert_eq!(result.is_err(), true);
}
