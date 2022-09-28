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
