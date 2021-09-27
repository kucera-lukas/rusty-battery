pub fn warning_message(threshold: u8) -> String {
    format!(
        "Battery percentage reached the {}% threshold, please unplug your charger",
        threshold,
    )
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
        ))
    }
}
