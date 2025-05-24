#[derive(Debug, PartialEq)]
pub enum RootTags {
    H1(H1),
    H2(H2),
    H3(H3),
    P(P),
    A(A),
    Img(Img),
    Li(Vec<Li>),
    Pre(Pre),
}

#[derive(Debug, PartialEq)]
pub struct H1(pub String);

#[derive(Debug, PartialEq)]
pub struct H2(pub String);

#[derive(Debug, PartialEq)]
pub struct H3(pub String);

#[derive(Debug, PartialEq)]
pub struct P(pub Vec<Contents>);

#[derive(Debug, PartialEq)]
pub enum Contents {
    Text(Text),
    Strong(Strong),
    Em(Em),
    Code(Code),
}

#[derive(Debug, PartialEq)]
pub struct Text(pub String);

#[derive(Debug, PartialEq)]
pub struct Strong(pub String);

#[derive(Debug, PartialEq)]
pub struct Em(pub String);

#[derive(Debug, PartialEq)]
pub struct Code(pub String);

#[derive(Debug, PartialEq)]
struct A {
    href: String,
    text: String,
}

#[derive(Debug, PartialEq)]
struct Img {
    src: String,
    alt: String,
}

#[derive(Debug, PartialEq)]
pub struct Li {
    pub list_type: ListTypes,
    pub indent: usize,
    pub contents: Vec<Contents>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ListTypes {
    Ul,
    Ol,
}

impl ListTypes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Ul => "ul",
            Self::Ol => "ol",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Pre(pub String);

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    output: Vec<RootTags>,
    indent: usize,
}

impl Lexer {
    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            return None;
        }
        Some(self.input[self.position + 1])
    }

    fn next_char(&mut self) -> Option<char> {
        self.position += 1;
        if self.position >= self.input.len() {
            return None;
        }
        Some(self.current_char())
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            if !self.current_char().is_ascii_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn read_to_eol(&mut self) -> String {
        let mut text = String::new();
        while self.position < self.input.len() {
            let c = self.current_char();
            if c == '\n' {
                break;
            }
            text.push(c);
            self.next_char();
        }
        text
    }
}

pub fn tokenize(input: &str) -> Vec<RootTags> {
    let mut lexer = Lexer {
        input: input.chars().collect(),
        position: 0,
        output: Vec::new(),
        indent: 0,
    };

    while lexer.position < lexer.input.len() {
        let c = lexer.current_char();
        match c {
            '#' => tokenize_heading(&mut lexer),

            '-' => tokenize_list(&mut lexer),

            '1' => tokenize_ordered_list(&mut lexer),
            '\t' | ' ' => {
                lexer.indent += 1;
                lexer.next_char();
            }
            '\n' => {
                lexer.indent = 0;
                lexer.next_char();
            }
            _ => {
                let text = lexer.read_to_eol();
                if text.starts_with("```") {
                    tokenize_codeblock(&mut lexer);
                } else {
                    tokenize_p_contents(&text, &mut lexer);
                }
            }
        }
    }

    lexer.output
}

fn tokenize_heading(lexer: &mut Lexer) {
    if Some('#') == lexer.next_char() {
        if Some('#') == lexer.next_char() {
            lexer.next_char();
            lexer.skip_whitespace();
            let text = lexer.read_to_eol();
            lexer.output.push(RootTags::H3(H3(text)));
        } else {
            lexer.skip_whitespace();
            let text = lexer.read_to_eol();
            lexer.output.push(RootTags::H2(H2(text)));
        }
    } else {
        lexer.skip_whitespace();
        let text = lexer.read_to_eol();
        lexer.output.push(RootTags::H1(H1(text)));
    }
}

fn tokenize_list(lexer: &mut Lexer) {
    lexer.next_char();
    lexer.skip_whitespace();
    let text = lexer.read_to_eol();
    let next_li = Li {
        list_type: ListTypes::Ul,
        indent: lexer.indent,
        contents: vec![Contents::Text(Text(text))],
    };
    if let Some(RootTags::Li(lists)) = lexer.output.last_mut() {
        lists.push(next_li);
    } else {
        lexer.output.push(RootTags::Li(vec![next_li]));
    }
}

fn tokenize_ordered_list(lexer: &mut Lexer) {
    if lexer.peek_char() == Some('.') {
        lexer.next_char();
        lexer.next_char();
        lexer.skip_whitespace();
        let text = lexer.read_to_eol();
        let next_li = Li {
            list_type: ListTypes::Ol,
            indent: lexer.indent,
            contents: vec![Contents::Text(Text(text))],
        };
        if let Some(RootTags::Li(lists)) = lexer.output.last_mut() {
            lists.push(next_li);
        } else {
            lexer.output.push(RootTags::Li(vec![next_li]));
        }
    } else {
        let text = lexer.read_to_eol();
        lexer
            .output
            .push(RootTags::P(P(vec![Contents::Text(Text(text))])))
    }
}

fn tokenize_codeblock(lexer: &mut Lexer) {
    let mut code = String::new();
    lexer.next_char();
    loop {
        let row = lexer.read_to_eol();
        lexer.next_char();
        if row.ends_with("```") {
            break;
        }
        code.push_str(&(row + "\n"));
    }
    lexer.output.push(RootTags::Pre(Pre(code)));
}

fn tokenize_p_contents(text: &str, lexer: &mut Lexer) {
    let mut p_contents = Vec::<Contents>::new();
    let mut text = text.chars();
    while let Some(c) = text.next() {
        match c {
            '`' => {
                let mut code = Code(String::new());
                while let Some(c) = text.next() {
                    if c == '`' {
                        break;
                    }
                    code.0.push(c);
                }
                p_contents.push(Contents::Code(code));
            }
            _ => {
                if let Some(Contents::Text(text)) = p_contents.last_mut() {
                    text.0.push(c);
                } else {
                    p_contents.push(Contents::Text(Text(c.to_string())));
                }
            }
        }
    }
    lexer.output.push(RootTags::P(P(p_contents)));
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::{Code, Contents, Li, ListTypes, Pre, RootTags, Text, H1, H2, H3, P};

    #[test]
    fn test_tokenize() {
        let tests = [
            (
                "テキスト`コード`",
                vec![RootTags::P(P(vec![
                    Contents::Text(Text("テキスト".to_string())),
                    Contents::Code(Code("コード".to_string())),
                ]))],
            ),
            ("# 見出し1", vec![RootTags::H1(H1("見出し1".to_string()))]),
            ("## 見出し2", vec![RootTags::H2(H2("見出し2".to_string()))]),
            ("### 見出し3", vec![RootTags::H3(H3("見出し3".to_string()))]),
            (
                "# 見出し1

## 見出し2

### 見出し3",
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::H3(H3("見出し3".to_string())),
                ],
            ),
            (
                "段落\n",
                vec![RootTags::P(P(vec![Contents::Text(Text(
                    "段落".to_string(),
                ))]))],
            ),
            (
                "# 見出し1

## 見出し2

### 見出し3

段落",
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::H3(H3("見出し3".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落".to_string()))])),
                ],
            ),
            (
                "- リスト",
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ul,
                    indent: 0,
                    contents: vec![Contents::Text(Text("リスト".to_string()))],
                }])],
            ),
            (
                "- リスト1
- リスト2",
                vec![RootTags::Li(vec![
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
                ])],
            ),
            (
                "- リスト1
- リスト2
\t- リスト2-1
\t- リスト2-2
",
                vec![RootTags::Li(vec![
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
                ])],
            ),
            (
                "- リスト1
- リスト2
  - リスト2-1
    - リスト2-1-1
  - リスト2-2
- リスト3
",
                vec![RootTags::Li(vec![
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
                        indent: 2,
                        contents: vec![Contents::Text(Text("リスト2-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 4,
                        contents: vec![Contents::Text(Text("リスト2-1-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 2,
                        contents: vec![Contents::Text(Text("リスト2-2".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ul,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト3".to_string()))],
                    },
                ])],
            ),
            (
                "```
console.log('Hello, world!');
```
",
                vec![RootTags::Pre(Pre(
                    "console.log('Hello, world!');\n".to_string()
                ))],
            ),
            (
                "```
const a = 1;
const b = 2;
add(a, b);
```
",
                vec![RootTags::Pre(Pre(
                    "const a = 1;\nconst b = 2;\nadd(a, b);\n".to_string(),
                ))],
            ),
            (
                "# 見出し1
## 見出し2

段落1
段落2

- リスト1
- リスト2
- リスト3

段落3
",
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落1".to_string()))])),
                    RootTags::P(P(vec![Contents::Text(Text("段落2".to_string()))])),
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
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト3".to_string()))],
                        },
                    ]),
                    RootTags::P(P(vec![Contents::Text(Text("段落3".to_string()))])),
                ],
            ),
            (
                "1. リスト",
                vec![RootTags::Li(vec![Li {
                    list_type: ListTypes::Ol,
                    indent: 0,
                    contents: vec![Contents::Text(Text("リスト".to_string()))],
                }])],
            ),
            (
                "1リストではない段落",
                vec![RootTags::P(P(vec![Contents::Text(Text(
                    "1リストではない段落".to_string(),
                ))]))],
            ),
            (
                "1. リスト1
1. リスト2",
                vec![RootTags::Li(vec![
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト2".to_string()))],
                    },
                ])],
            ),
            (
                "1. リスト1
1. リスト2
\t1. リスト2-1
\t1. リスト2-2
",
                vec![RootTags::Li(vec![
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
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
            ),
            (
                "1. リスト1
1. リスト2
  1. リスト2-1
    1. リスト2-1-1
  1. リスト2-2
1. リスト3
",
                vec![RootTags::Li(vec![
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト2".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 2,
                        contents: vec![Contents::Text(Text("リスト2-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 4,
                        contents: vec![Contents::Text(Text("リスト2-1-1".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 2,
                        contents: vec![Contents::Text(Text("リスト2-2".to_string()))],
                    },
                    Li {
                        list_type: ListTypes::Ol,
                        indent: 0,
                        contents: vec![Contents::Text(Text("リスト3".to_string()))],
                    },
                ])],
            ),
            (
                "# 見出し1
## 見出し2

段落1

```
console.log('Hello, world!');
```

段落2

```
console.log('Hello, world!');
```

1. リスト1
1. リスト2
1. リスト3
- リスト1
- リスト2
- リスト3

```
console.log('Hello, world!');
```

段落3
",
                vec![
                    RootTags::H1(H1("見出し1".to_string())),
                    RootTags::H2(H2("見出し2".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落1".to_string()))])),
                    RootTags::Pre(Pre("console.log('Hello, world!');\n".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落2".to_string()))])),
                    RootTags::Pre(Pre("console.log('Hello, world!');\n".to_string())),
                    RootTags::Li(vec![
                        Li {
                            list_type: ListTypes::Ol,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト1".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ol,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト2".to_string()))],
                        },
                        Li {
                            list_type: ListTypes::Ol,
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト3".to_string()))],
                        },
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
                            indent: 0,
                            contents: vec![Contents::Text(Text("リスト3".to_string()))],
                        },
                    ]),
                    RootTags::Pre(Pre("console.log('Hello, world!');\n".to_string())),
                    RootTags::P(P(vec![Contents::Text(Text("段落3".to_string()))])),
                ],
            ),
        ];

        for (input, output) in tests {
            assert_eq!(tokenize(input), output);
        }
    }
}
