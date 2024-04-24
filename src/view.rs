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
      $content = re.replace_all(&$content, $x).into_owned();
    )*
};
}

pub fn view(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Something went wrong reading the file")
}
