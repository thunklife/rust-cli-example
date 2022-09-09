use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use std::io;
use serde_json;

const DB_PATH: &str = r#"./data/db.json"#;

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    categorgy: String,
    age: usize,
    created_at: DateTime<Utc>
}

#[derive(Error, Debug)]
pub enum Error {
   #[error("error reading from the DB filei: {0}")]
   ReadDBError(#[from] io::Error),
   #[error("error parsing DB file: {0}")]
   ParseDBError(#[from] serde_json::Error)
}

enum Event<I> {
    Input(I),
    Tick,
}
fn main() {
    println!("Hello, world!");
}
