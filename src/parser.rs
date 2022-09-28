use std::ops;

const THRESHOLD_RANGE: ops::RangeInclusive<u8> = 0..=100;

pub fn threshold(s: &str) -> Result<u8, String> {
    let threshold = s.parse::<u8>().map_err(|e| e.to_string())?;

    if THRESHOLD_RANGE.contains(&threshold) {
        Ok(threshold)
    } else {
        Err(format!(
            "not in range {}-{}",
            THRESHOLD_RANGE.start(),
            THRESHOLD_RANGE.end()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! threshold_tests_ok {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (value, expected) = $value;

                let result = threshold(value);

                assert!(result.is_ok());
                assert_eq!(result, Ok(expected));
            }
        )*
        }
    }

    macro_rules! threshold_tests_err {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let value = $value;

                let result = threshold(value);

                assert!(result.is_err());
            }
        )*
        }
    }

    threshold_tests_ok! {
        test_threshold_start: ("0", 0),
        test_threshold_middle: ("50", 50),
        test_threshold_end: ("100", 100),
    }

    threshold_tests_err! {
        test_threshold_negative_number: "-20",
        test_threshold_large_number: "1500",
        test_threshold_empty: "",
        test_threshold_whitespace: " ",
        test_threshold_multiple_whitespace: "    ",
        test_threshold_invalid_digit: "r",
        test_threshold_multiple_invalid_digit: "rusty-battery",
        test_threshold_number_and_invalid_digit: "1r",
        test_threshold_number_and_multiple_invalid_digit: "1rusty-battery",
    }
}
