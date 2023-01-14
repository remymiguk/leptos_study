use std::fmt;

// https://css-tricks.com/everything-you-ever-wanted-to-know-about-inputmode/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Empty,
    None,
    Numeric,
    Decimal,
    Email,
    Url,
    Search,
    Tel,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            InputMode::Empty => String::from("empty"),
            InputMode::None => String::from("none"),
            InputMode::Numeric => String::from("numeric"),
            InputMode::Decimal => String::from("decimal"),
            InputMode::Email => String::from("email"),
            InputMode::Url => String::from("url"),
            InputMode::Search => String::from("search"),
            InputMode::Tel => String::from("tel"),
        };
        write!(f, "{s}")
    }
}
