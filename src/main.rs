use serde::{Deserialize, Serialize};
use std::env::args;

use crate::{lexer::tokenize, parser::parse, view::view};

mod lexer;
mod parser;
mod view;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    slug: String,
    title: String,
    created_at: String,
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 5 {
        panic!(
            "Usage: {} <markdown> <template> <data_json> <slug>",
            args[0]
        );
    }
    let content = parse(tokenize(&args[1]));
    let title = content
        .lines()
        .next()
        .unwrap()
        .replace("<h1>", "")
        .replace("</h1>", "");
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut data: Vec<Data> =
        serde_json::from_str(std::fs::read_to_string(&args[3]).unwrap().as_str()).unwrap();
    if let Some(article) = data.iter_mut().find(|data| data.slug == args[4]) {
        article.title = title.clone();
    } else {
        data.push(Data {
            slug: args[4].clone(),
            title: title.clone(),
            created_at: date.clone(),
        });
    }
    std::fs::write(&args[3], serde_json::to_string_pretty(&data).unwrap()).unwrap();

    let mut html = view(&args[2]);
    compact!(html, title, content, date);
    print!("{}", html);
}
