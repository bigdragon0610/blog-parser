use crate::lexer::{Contents, RootTags};

struct Parser {
    html: String,
}

impl Parser {
    fn push_html(&mut self, tag_name: &str, content: &str) {
        self.html
            .push_str(&format!("<{}>{}</{}>\n", tag_name, content, tag_name));
    }
}

fn parse(tags: Vec<RootTags>) -> String {
    let mut parser = Parser {
        html: String::new(),
    };

    for (i, tag) in tags.iter().enumerate() {
        match tag {
            RootTags::H1(h1) => parser.push_html("h1", &h1.0),
            RootTags::H2(h2) => parser.push_html("h2", &h2.0),
            RootTags::H3(h3) => parser.push_html("h3", &h3.0),
            RootTags::P(p) => {
                let mut p_contents = String::new();
                for content in &p.0 {
                    match content {
                        Contents::Text(text) => p_contents.push_str(&text.0),
                        _ => panic!(),
                    }
                }
                parser.push_html("p", &p_contents)
            }
            _ => panic!(),
        }
    }

    parser.html
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Contents, RootTags, Text, H1, H2, H3, P},
        parser::parse,
    };

    #[test]
    fn test_parse() {
        let tests = [
            (
                vec![RootTags::H1(H1("見出し1".to_string()))],
                "<h1>見出し1</h1>\n",
            ),
            (
                vec![RootTags::H2(H2("見出し2".to_string()))],
                "<h2>見出し2</h2>\n",
            ),
            (
                vec![RootTags::H3(H3("見出し3".to_string()))],
                "<h3>見出し3</h3>\n",
            ),
            (
                vec![RootTags::P(P(vec![Contents::Text(Text(
                    "段落".to_string(),
                ))]))],
                "<p>段落</p>\n",
            ),
            (
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::H3(H3("見出し3".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落".to_string()))])),
                ],
                "<h1>見出し1</h1>
<h2>見出し2</h2>
<h3>見出し3</h3>
<p>段落</p>
",
            ),
        ];

        for (input, html) in tests {
            assert_eq!(parse(input), html);
        }
    }
}
