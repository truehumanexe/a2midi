use lazy_regex::{self, Regex, Lazy, Captures};


/// A wrapper around useful regexes for gui text input
/// separated from the app editor for easy testing.
pub struct AppRegex;

impl AppRegex{

    pub const TEMPO:&Lazy<Regex, fn() -> Regex> = lazy_regex::regex!(r"^(\d{2,3})(\.\d{1,2}){0,1}");
    pub const TIME_SIG:&Lazy<Regex, fn() -> Regex> = lazy_regex::regex!(r"^(\d{1,2})(\/)(\d{1,2})");

    pub fn validate_all(captures:&Option<Captures>) -> bool{
        match captures {
            // first value of a successful match always returns some, so skip it
            Some(c) => c.iter().skip(1).all(|m| m.is_some()),
            None => false,
        }
    }

    pub fn validate_n_(captures: &Option<Captures>, n:usize) -> bool{
        match captures {
            Some(c) => c.get(n).is_some(),
            None => false,
        }
    }

}
