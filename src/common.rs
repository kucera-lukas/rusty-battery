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

pub fn print_slice<T>(slice: &[T])
where
    T: Display,
{
    slice.iter().for_each(|item| println!("{}", item));
}

pub fn format_option<T>(option: &Option<T>) -> String
where
    T: Display,
{
    match option {
        None => "None".into(),
        Some(value) => format!("{}", value),
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
