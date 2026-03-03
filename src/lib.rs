pub mod io;
pub mod models;
pub mod shifter;

use anyhow::{anyhow, Result};
use std::time::Duration;

pub fn parse_offset(offset_str: &str) -> Result<i64> {
    let offset_str = offset_str.trim();
    if offset_str.is_empty() {
        return Err(anyhow!("Offset cannot be empty"));
    }

    let (sign, duration_part) = if offset_str.starts_with('-') {
        (-1, &offset_str[1..])
    } else if offset_str.starts_with('+') {
        (1, &offset_str[1..])
    } else {
        (1, offset_str)
    };

    // If only a number is provided, assume seconds
    let final_duration_str = if duration_part.chars().all(|c| c.is_digit(10) || c == '.') {
        format!("{}s", duration_part)
    } else {
        duration_part.to_string()
    };

    let duration: Duration = final_duration_str.parse::<humantime::Duration>()?.into();
    Ok(sign * duration.as_millis() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_offset() {
        assert_eq!(parse_offset("+1.5s").unwrap(), 1500);
        assert_eq!(parse_offset("-500ms").unwrap(), -500);
        assert_eq!(parse_offset("2.0").unwrap(), 2000);
        assert_eq!(parse_offset("-2").unwrap(), -2000);
    }

    #[test]
    fn test_parse_offset_errors() {
        assert!(parse_offset("").is_err());
        assert!(parse_offset("foo").is_err());
        assert!(parse_offset("+ms").is_err());
    }
}
