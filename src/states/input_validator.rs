use dyn_clonable::clonable;
use log::warn;
use rust_decimal::Decimal;

#[clonable]
pub trait InputValidator: std::fmt::Debug + Clone {
    fn value(&self, previous_input: String, current_input: String) -> String;
}

#[derive(Debug, Clone)]
pub struct DecimalValidator {
    scale: u32,
}

impl DecimalValidator {
    pub fn new(scale: u32) -> Self {
        Self { scale }
    }
}

impl InputValidator for DecimalValidator {
    fn value(&self, previous_input: String, current_input: String) -> String {
        let mut decimal = match Decimal::from_str_exact(&current_input) {
            Ok(decimal) => decimal,
            Err(_) => {
                warn!("defining previous ***");
                return previous_input;
            }
        };
        decimal.set_scale(self.scale).unwrap();
        decimal.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_validator_test() {
        let validator = DecimalValidator::new(2);

        assert_eq!(
            validator.value(String::from(""), String::from("")),
            String::from("")
        );
        assert_eq!(
            validator.value(String::from("1"), String::from("1")),
            String::from("0.01")
        );
        assert_eq!(
            validator.value(String::from("0.012"), String::from("0.012")),
            String::from("0.12")
        );
        assert_eq!(
            validator.value(String::from("0.123"), String::from("0.123")),
            String::from("1.23")
        );
        assert_eq!(
            validator.value(String::from("123.45"), String::from("123.45")),
            String::from("123.45")
        );
    }
}
