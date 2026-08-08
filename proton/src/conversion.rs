use thiserror::Error;

/// Errors that can occur during inch to meter conversion.
#[derive(Debug, Error)]
pub enum InchesToMetersError {
    /// Error when the input value is negative.
    #[error("Negative value not allowed: {0}")]
    NegativeValue(f32),

    /// Error when the input value is too large.
    #[error("Value too large: {0}")]
    TooLarge(f32),

    /// Error when the input value is not a finite number.
    #[error("Value is not finite: {0}")]
    NotFinite(f32),
}

/// Convert inches to meters.
///
/// # Arguments
///
/// * `inches` - The value in inches to convert
///
/// # Returns
///
/// * `Ok(f32)` - The equivalent value in meters
/// * `Err(InchesToMetersError)` - If the input is invalid
///
/// # Examples
///
/// ```
/// use proton::convert_inches_to_meters;
///
/// let result = convert_inches_to_meters(39.37).expect("Valid input");
/// assert_eq!(result, 1.0);
/// ```
pub fn convert_inches_to_meters(inches: f32) -> Result<f32, InchesToMetersError> {
    if !inches.is_finite() {
        return Err(InchesToMetersError::NotFinite(inches));
    }

    if inches < 0.0 {
        return Err(InchesToMetersError::NegativeValue(inches));
    }

    // Arbitrary limit to prevent extremely large values
    if inches > 1_000_000.0 {
        return Err(InchesToMetersError::TooLarge(inches));
    }

    const INCHES_PER_METER: f32 = 39.37;
    Ok(inches / INCHES_PER_METER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_conversion() {
        assert_eq!(convert_inches_to_meters(39.37).unwrap(), 1.0);
        assert_eq!(convert_inches_to_meters(0.0).unwrap(), 0.0);
        assert_eq!(convert_inches_to_meters(1.0).unwrap(), 1.0 / 39.37);
    }

    #[test]
    fn test_negative_value() {
        assert!(matches!(
            convert_inches_to_meters(-1.0),
            Err(InchesToMetersError::NegativeValue(_))
        ));
    }

    #[test]
    fn test_too_large_value() {
        assert!(matches!(
            convert_inches_to_meters(2_000_000.0),
            Err(InchesToMetersError::TooLarge(_))
        ));
    }

    #[test]
    fn test_non_finite_value() {
        assert!(matches!(
            convert_inches_to_meters(f32::NAN),
            Err(InchesToMetersError::NotFinite(_))
        ));
        assert!(matches!(
            convert_inches_to_meters(f32::INFINITY),
            Err(InchesToMetersError::NotFinite(_))
        ));
    }
}
