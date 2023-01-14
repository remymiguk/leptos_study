use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputIcon {
    User,
    Envelope,
    Key,
}

impl fmt::Display for InputIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            InputIcon::User => "fa-user",
            InputIcon::Envelope => "fa-envelope",
            InputIcon::Key => "fa-key",
        };
        write!(f, "{}", s)
    }
}
