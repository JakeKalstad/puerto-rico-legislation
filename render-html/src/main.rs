use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
struct History {
    description: String,
    date: chrono::naive::NaiveDate,
    document: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Measure {
    name: String,
    date: chrono::naive::NaiveDate,
    heading: String,
    authors: Vec<String>,
    history: Vec<History>,
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("../pr-legislation/measures.json")
        .expect("Something went wrong reading the file");
    let measures: Vec<Measure> = serde_json::from_str(&contents)?;
    Ok(for m in measures {
        print!("{}", m.name);
    })
}
