use crate::lexer::{Contents, ListTypes, RootTags, Text};

struct Parser {
    html: String,
}

impl Parser {
    fn push_html(&mut self, tag_name: &str, content: &str, new_line: bool) {
        self.html.push_str(&format!(
            "<{}>{}</{}>{}",
            tag_name,
            content,
            tag_name,
            if new_line { "\n" } else { "" }
        ));
    }
}

pub fn parse(tags: Vec<RootTags>) -> String {
    let mut parser = Parser {
        html: String::new(),
    };

    for tag in tags {
        match tag {
            RootTags::H1(h1) => parser.push_html("h1", &h1.0, true),
            RootTags::H2(h2) => parser.push_html("h2", &h2.0, true),
            RootTags::H3(h3) => parser.push_html("h3", &h3.0, true),
            RootTags::P(p) => {
                let mut p_contents = String::new();
                for content in &p.0 {
                    match content {
                        Contents::Text(text) => p_contents.push_str(&text.0),
                        Contents::Code(code) => {
                            p_contents.push_str(&format!("<code>{}</code>", code.0))
                        }
                        Contents::Bold(bold) => p_contents.push_str(&format!("<b>{}</b>", bold.0)),
                        Contents::Italic(italic) => {
                            p_contents.push_str(&format!("<i>{}</i>", italic.0))
                        }
                    }
                }
                parser.push_html("p", &p_contents, true)
            }
            RootTags::Li(lists) => {
                let mut stack = Vec::<(usize, ListTypes)>::new();
                stack.push((lists[0].indent, lists[0].list_type));
                parser.html.push_str(&format!(
                    "<{}>\n<li>{}",
                    lists[0].list_type.to_string(),
                    lists[0]
                        .contents
                        .iter()
                        .fold(String::new(), |mut acc, content| {
                            acc += &match content {
                                Contents::Text(Text(text)) => text.to_string(),
                                Contents::Code(code) => format!("<code>{}</code>", code.0),
                                Contents::Bold(bold) => format!("<b>{}</b>", bold.0),
                                Contents::Italic(italic) => format!("<i>{}</i>", italic.0),
                            };
                            acc
                        })
                ));

                for li in lists.iter().skip(1) {
                    while let Some(&(indent, list_type)) = stack.last() {
                        match indent.cmp(&li.indent) {
                            std::cmp::Ordering::Equal => {
                                parser.html.push_str("</li>\n");
                                break;
                            }
                            std::cmp::Ordering::Less => {
                                parser.html.push_str(&format!(
                                    "\n{}<{}>\n",
                                    "\t".repeat(li.indent),
                                    li.list_type.to_string(),
                                ));
                                stack.push((li.indent, li.list_type));
                                break;
                            }
                            std::cmp::Ordering::Greater => {
                                stack.pop();
                                let &(second_last_indent, _) = stack.last().unwrap();
                                parser.html.push_str(&format!(
                                    "</li>\n{}</{}>\n{}",
                                    "\t".repeat(indent),
                                    list_type.to_string(),
                                    "\t".repeat(second_last_indent)
                                ));
                            }
                        }
                    }
                    parser.html.push_str(&format!(
                        "{}<li>{}",
                        "\t".repeat(li.indent),
                        li.contents.iter().fold(String::new(), |mut acc, content| {
                            acc += &match content {
                                Contents::Text(Text(text)) => text.to_string(),
                                Contents::Code(code) => format!("<code>{}</code>", code.0),
                                Contents::Bold(bold) => format!("<b>{}</b>", bold.0),
                                Contents::Italic(italic) => format!("<i>{}</i>", italic.0),
                            };
                            acc
                        })
                    ))
                }
                while let Some((indent, list_type)) = stack.pop() {
                    parser.html.push_str(&format!(
                        "</li>\n{}</{}>\n",
                        "\t".repeat(indent),
                        list_type.to_string()
                    ));
                }
            }
            RootTags::Pre(pre) => parser
                .html
                .push_str(&format!("<pre><code>{}</code></pre>\n", pre.0)),
            _ => panic!(),
        }
    }

    parser.html
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Bold, Code, Contents, Italic, Li, ListTypes, Pre, RootTags, Text, H1, H2, H3, P},
        parser::parse,
    };

    #[test]
    fn test_parse() {
        let tests = [
            (
                vec![RootTags::P(P(vec![
                    Contents::Text(Text("テキスト".to_string())),
                    Contents::Code(Code("コード".to_string())),
                ]))],
                "<p>テキスト<code>コード</code></p>\n",
            ),
            (
                vec![RootTags::P(P(vec![Contents::Italic(Italic(
                    "イタリック".to_string(),
                ))]))],
                "<p><i>イタリック</i></p>\n",
            ),
            (
                vec![RootTags::P(P(vec![Contents::Bold(Bold(
                    "ボールド".to_string(),
                ))]))],
                "<p><b>ボールド</b></p>\n",
            ),
            (
                vec![RootTags::P(P(vec![
                    Contents::Italic(Italic("イタリック".to_string())),
                    Contents::Bold(Bold("ボールド".to_string())),
                ]))],
                "<p><i>イタリック</i><b>ボールド</b></p>\n",
            ),
            (
                vec![RootTags::P(P(vec![
                    Contents::Text(Text("テキスト".to_string())),
                    Contents::Italic(Italic("イタリック".to_string())),
                    Contents::Code(Code("コード".to_string())),
                    Contents::Bold(Bold("ボールド".to_string())),
                ]))],
                "<p>テキスト<i>イタリック</i><code>コード</code><b>ボールド</b></p>\n",
            ),
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
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ul,
                    indent: 0,
                    contents: vec![Contents::Text(Text("リスト".to_string()))],
                }])],
                "<ul>\n<li>リスト</li>\n</ul>\n",
            ),
            (
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ol,
                    indent: 0,
                    contents: vec![Contents::Text(Text("リスト".to_string()))],
                }])],
                "<ol>\n<li>リスト</li>\n</ol>\n",
            ),
            (
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ol,
                    indent: 0,
                    contents: vec![Contents::Text(Text("リスト".to_string()))],
                }])],
                "<ol>\n<li>リスト</li>\n</ol>\n",
            ),
            (
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ul,
                    indent: 0,
                    contents: vec![
                        Contents::Text(Text("テキスト".to_string())),
                        Contents::Italic(Italic("イタリック".to_string())),
                        Contents::Code(Code("コード".to_string())),
                        Contents::Bold(Bold("ボールド".to_string())),
                    ],
                }])],
                "<ul>\n<li>テキスト<i>イタリック</i><code>コード</code><b>ボールド</b></li>\n</ul>\n",
            ),
            (
                vec![RootTags::Li(vec![
                  Li {
                    list_type: ListTypes::Ul,
                    indent: 0,
                    contents: vec![
                        Contents::Text(Text("テキスト".to_string())),
                        Contents::Italic(Italic("イタリック".to_string())),
                        Contents::Code(Code("コード".to_string())),
                        Contents::Bold(Bold("ボールド".to_string())),
                    ],
                },
                  Li {
                    list_type: ListTypes::Ul,
                    indent: 2,
                    contents: vec![
                        Contents::Text(Text("テキスト".to_string())),
                        Contents::Italic(Italic("イタリック".to_string())),
                        Contents::Code(Code("コード".to_string())),
                        Contents::Bold(Bold("ボールド".to_string())),
                    ],
                },
                ])],
                "<ul>
<li>テキスト<i>イタリック</i><code>コード</code><b>ボールド</b>
\t\t<ul>
\t\t<li>テキスト<i>イタリック</i><code>コード</code><b>ボールド</b></li>
\t\t</ul>
</li>
</ul>\n",
            ),
            (
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ol,
                    indent: 0,
                    contents: vec![
                        Contents::Text(Text("テキスト".to_string())),
                        Contents::Italic(Italic("イタリック".to_string())),
                        Contents::Code(Code("コード".to_string())),
                        Contents::Bold(Bold("ボールド".to_string())),
                    ],
                }])],
                "<ol>\n<li>テキスト<i>イタリック</i><code>コード</code><b>ボールド</b></li>\n</ol>\n",
            ),
            (
                vec![RootTags::Pre(Pre(
                    "console.log('Hello, world!');\n".to_string()
                ))],
                "<pre><code>console.log('Hello, world!');\n</code></pre>\n",
            ),
            (
                vec![RootTags::Pre(Pre(
                    "const a = 1;\nconst b = 2;\nadd(a, b);\n".to_string(),
                ))],
                "<pre><code>const a = 1;
const b = 2;
add(a, b);
</code></pre>
",
            ),
            (
                vec![RootTags::Li(vec![
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 1,
                        contents: vec![Contents::Text(Text("リスト1-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 1,
                        contents: vec![Contents::Text(Text("リスト1-2".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト2".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 1,
                        contents: vec![Contents::Text(Text("リスト2-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 1,
                        contents: vec![Contents::Text(Text("リスト2-2".to_string()))],
                    },
                ])],
                "<ul>
<li>リスト1
\t<ul>
\t<li>リスト1-1</li>
\t<li>リスト1-2</li>
\t</ul>
</li>
<li>リスト2
\t<ol>
\t<li>リスト2-1</li>
\t<li>リスト2-2</li>
\t</ol>
</li>
</ul>
",
            ),
            (
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落1".to_string()))])),
                    RootTags::H3(H3("見出し3".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落2".to_string()))])),
                    RootTags::Pre(Pre("console.log('Hello, world!');\n".to_string())),
                    RootTags::Li(vec![
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト1".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト2".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 1,
                            contents: vec![Contents::Text(Text("リスト2-1".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 1,
                            contents: vec![Contents::Text(Text("リスト2-2".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 2,
                            contents: vec![Contents::Text(Text("リスト2-2-1".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 1,
                            contents: vec![Contents::Text(Text("リスト2-3".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ul,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト3".to_string()))],
                        },
                    ]),
                    RootTags::Pre(Pre("console.log('Hello, world!');\n".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落3".to_string()))])),
                ],
                "<h1>見出し1</h1>
<h2>見出し2</h2>
<p>段落1</p>
<h3>見出し3</h3>
<p>段落2</p>
<pre><code>console.log('Hello, world!');
</code></pre>
<ul>
<li>リスト1</li>
<li>リスト2
\t<ul>
\t<li>リスト2-1</li>
\t<li>リスト2-2
\t\t<ul>
\t\t<li>リスト2-2-1</li>
\t\t</ul>
\t</li>
\t<li>リスト2-3</li>
\t</ul>
</li>
<li>リスト3</li>
</ul>
<pre><code>console.log('Hello, world!');
</code></pre>
<p>段落3</p>
",
            ),
        ];

        for (input, html) in tests {
            assert_eq!(parse(input), html);
        }
    }
}
