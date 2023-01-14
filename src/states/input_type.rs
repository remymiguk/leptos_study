use std::fmt;

// https://html.spec.whatwg.org/multipage/input.html#the-step-attribute
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Time,
    Url,
    Week,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            InputType::Button => String::from("button"),
            InputType::Checkbox => String::from("checkbox"),
            InputType::Color => String::from("color"),
            InputType::Date => String::from("date"),
            InputType::DatetimeLocal => String::from("datetime-local"),
            InputType::Email => String::from("email"),
            InputType::File => String::from("file"),
            InputType::Hidden => String::from("hidden"),
            InputType::Image => String::from("image"),
            InputType::Month => String::from("month"),
            InputType::Week => String::from("week"),
            InputType::Range => String::from("range"),
            InputType::Number => String::from("number"),
            InputType::Password => String::from("password"),
            InputType::Radio => String::from("radio"),
            InputType::Reset => String::from("reset"),
            InputType::Search => String::from("search"),
            InputType::Submit => String::from("submit"),
            InputType::Tel => String::from("tel"),
            InputType::Text => String::from("text"),
            InputType::Time => String::from("time"),
            InputType::Url => String::from("url"),
        };
        write!(f, "{}", s)
    }
}
