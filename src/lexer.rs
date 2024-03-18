#[derive(Debug, PartialEq)]
enum RootTags {
    H1(H1),
    H2(H2),
    H3(H3),
    P(P),
    A(A),
    Img(Img),
    Ul(Ul),
    Li(Li),
    Pre(Pre),
}

#[derive(Debug, PartialEq)]
struct H1(String);

#[derive(Debug, PartialEq)]
struct H2(String);

#[derive(Debug, PartialEq)]
struct H3(String);

#[derive(Debug, PartialEq)]
struct P(Vec<PContent>);

#[derive(Debug, PartialEq)]
enum PContent {
    Text(Text),
    Strong(Strong),
    Em(Em),
    Code(Code),
}

#[derive(Debug, PartialEq)]
struct Text(String);

#[derive(Debug, PartialEq)]
struct Strong(String);

#[derive(Debug, PartialEq)]
struct Em(String);

#[derive(Debug, PartialEq)]
struct Code(String);

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
enum LiContent {
    Text(Text),
    Ul(Ul),
    Ol(Ol),
}

#[derive(Debug, PartialEq)]
struct Ul(Vec<Li>);

#[derive(Debug, PartialEq)]
struct Ol(Vec<Li>);

#[derive(Debug, PartialEq)]
struct Li(Vec<LiContent>);

#[derive(Debug, PartialEq)]
struct Pre(String);

struct Lexer {
    input: Vec<char>,
    position: usize,
    output: Vec<RootTags>,
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

fn tokenize(input: &str) -> Vec<RootTags> {
    let mut lexer = Lexer {
        input: input.chars().collect(),
        position: 0,
        output: Vec::new(),
    };

    while lexer.position < lexer.input.len() {
        let c = lexer.current_char();
        match c {
            '#' => {
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
            '\n' => {
                lexer.next_char();
            }
            _ => {
                let text = lexer.read_to_eol();
                lexer
                    .output
                    .push(RootTags::P(P(vec![PContent::Text(Text(text))])))
            }
        }
    }

    lexer.output
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::{PContent, RootTags, Text, H1, H2, H3, P};

    #[test]
    fn test_parse() {
        let tests = [
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
                vec![RootTags::P(P(vec![PContent::Text(Text(
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
                    RootTags::P(P(vec![PContent::Text(Text("段落".to_string()))])),
                ],
            ),
        ];

        for (input, output) in tests {
            assert_eq!(tokenize(input), output);
        }
    }
}
