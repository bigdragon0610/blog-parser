pub mod __export {
    pub use ::regex::Regex;
}
use std::fs;

#[macro_export]
macro_rules! compact {
($content:ident, $($x:ident),+) => {
    $(
      let re_string = format!(r"\{{\{{\s*\${}\s*\}}\}}", stringify!($x));
      let re = $crate::view::__export::Regex::new(&re_string).unwrap();
      let escaped = $x.replace("$", "$$"); // 置換文字列の$は特殊文字
      $content = re.replace_all(&$content, escaped).into_owned();
    )*
};
}

pub fn view(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compact() {
        let mut html =
            String::from("{{ $title }} - {{ $content }} - {{ $date }} - {{ $description }}");
        let title = "Title";
        let content = "Content";
        let date = "Date";
        let description = "Description";

        compact!(html, title, content, date, description);

        assert_eq!(html, "Title - Content - Date - Description");
    }

    #[test]
    fn test_compact_escape() {
        let mut output = String::from("{{ $text }}");
        let text = "$";
        compact!(output, text);
        assert_eq!(output, "$");

        let mut output = String::from("{{ $text }}");
        let text = "$sample";
        compact!(output, text);
        assert_eq!(output, "$sample");

        let mut output = String::from("{{ $text }}");
        let text = "$$sample $$sample";
        compact!(output, text);
        assert_eq!(output, "$$sample $$sample");
    }
}
