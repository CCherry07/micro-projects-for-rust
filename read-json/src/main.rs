use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn json_str_to_json(json_str: &str) -> Result<Article, serde_json::Error> {
    serde_json::from_str(json_str)
}

fn main() {
    let demo_json_str = fs::read_to_string("./demo.json").unwrap();
    let demo_json: Article = json_str_to_json(&demo_json_str).unwrap();
    println!("{}", demo_json.paragraphs[0].name)
}
