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

fn main() {
    let article = Article {
        article: String::from("hello world"),
        author: String::from("cherry"),
        paragraphs: vec![
            Paragraph {
                name: String::from("rust"),
            },
            Paragraph {
                name: String::from("javascript"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("json:{}", json)
}
