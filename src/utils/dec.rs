use rust_decimal::{Decimal, RoundingStrategy};

pub fn fix_scale(mut num: Decimal, digits: u32) -> Decimal {
    num.rescale(digits);
    // num = num.round_dp_with_strategy(digits, RoundingStrategy::ToZero);
    // let d_diff = digits - num.scale();
    // num *= Decimal::new(10_u32.pow(d_diff) as i64, 0);
    // num.set_scale(digits).unwrap();
    num
}

#[cfg(test)]
mod tests {
    use crate::utils::dec::fix_scale;
    use rust_decimal_macros::dec;

    #[test]
    fn fix_scale_test() {
        assert_eq!(fix_scale(dec!(2), 2).to_string(), String::from("2.00"));
        assert_eq!(fix_scale(dec!(2), 4).to_string(), String::from("2.0000"));
        assert_eq!(fix_scale(dec!(2.1), 2).to_string(), String::from("2.10"));
        assert_eq!(fix_scale(dec!(2.01), 2).to_string(), String::from("2.01"));
        assert_eq!(fix_scale(dec!(2.01), 4).to_string(), String::from("2.0100"));
        assert_eq!(fix_scale(dec!(2.001), 2).to_string(), String::from("2.00"));
        assert_eq!(fix_scale(dec!(2.009), 2).to_string(), String::from("2.01"));
        assert_eq!(fix_scale(dec!(2.001), 0).to_string(), String::from("2"));
    }
}
