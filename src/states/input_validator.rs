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
        decimal.rescale(self.scale);
        decimal.to_string()
    }
}
