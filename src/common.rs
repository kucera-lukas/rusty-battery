use std::fmt::Display;
use std::result;

pub fn warning_message(threshold: u8) -> String {
    format!(
        "Battery percentage reached the {}% threshold, please unplug your charger",
        threshold,
    )
}

pub fn warn_on_err<T, E>(result: result::Result<T, E>)
where
    E: Display,
{
    match result {
        Ok(_) => {}
        Err(e) => log::warn!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_message() {
        let threshold = 50;

        let result = warning_message(threshold);

        assert_eq!(result, format!(
            "Battery percentage reached the {}% threshold, please unplug your charger",
            threshold,
        ));
    }
}
