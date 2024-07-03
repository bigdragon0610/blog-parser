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
