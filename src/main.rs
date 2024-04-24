use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, env::args};

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

    let Params {
        description,
        skip_cnt,
    } = parse_params(&args[1]);

    let content = parse(tokenize(&args[1][skip_cnt..]));

    let title = content
        .lines()
        .next()
        .unwrap()
        .replace("<h1>", "")
        .replace("</h1>", "");

    let mut data: VecDeque<Data> =
        serde_json::from_str(std::fs::read_to_string(&args[3]).unwrap().as_str()).unwrap();
    let date;
    if let Some(article) = data.iter_mut().find(|data| data.slug == args[4]) {
        article.title = title.clone();
        date = article.created_at.clone();
    } else {
        date = chrono::Local::now().format("%Y-%m-%d").to_string();
        data.push_front(Data {
            slug: args[4].clone(),
            title: title.clone(),
            created_at: date.clone(),
        });
    }
    std::fs::write(&args[3], serde_json::to_string_pretty(&data).unwrap()).unwrap();

    let mut html = view(&args[2]);
    compact!(html, title, content, date, description);
    print!("{}", html);
}

struct Params<'a> {
    description: &'a str,
    skip_cnt: usize,
}

fn parse_params(markdown: &str) -> Params<'_> {
    let mut description = "";
    let mut skip_cnt = 0;
    let mut cnt = 0;
    for line in markdown.lines() {
        skip_cnt += line.len() + 1;
        if line.starts_with("---") {
            cnt += 1;
            if cnt == 2 {
                break;
            }
        } else if line.starts_with("description:") {
            description = line.trim_start_matches("description:").trim();
        }
    }
    if description.is_empty() {
        panic!("description not found in markdown");
    }
    Params {
        description,
        skip_cnt,
    }
}
