use proton::{convert_inches_to_meters, InchesToMetersError};

#[test]
fn test_integration_valid_conversions() {
    // Test known conversion values
    assert_eq!(convert_inches_to_meters(0.0).unwrap(), 0.0);
    assert_eq!(convert_inches_to_meters(39.37).unwrap(), 1.0);
    assert_eq!(convert_inches_to_meters(1.0).unwrap(), 1.0 / 39.37);
    assert_eq!(convert_inches_to_meters(78.74).unwrap(), 2.0);
}

#[test]
fn test_integration_error_conditions() {
    // Test negative value
    assert!(matches!(
        convert_inches_to_meters(-5.0),
        Err(InchesToMetersError::NegativeValue(_))
    ));

    // Test too large value
    assert!(matches!(
        convert_inches_to_meters(2_000_000.0),
        Err(InchesToMetersError::TooLarge(_))
    ));

    // Test non-finite values
    assert!(matches!(
        convert_inches_to_meters(f32::NAN),
        Err(InchesToMetersError::NotFinite(_))
    ));
    assert!(matches!(
        convert_inches_to_meters(f32::INFINITY),
        Err(InchesToMetersError::NotFinite(_))
    ));
    assert!(matches!(
        convert_inches_to_meters(f32::NEG_INFINITY),
        Err(InchesToMetersError::NotFinite(_))
    ));
}

#[test]
fn test_integration_round_trip() {
    // Test that converting from inches to meters and back gives original value
    let original_values = [0.0, 1.0, 10.0, 100.0, 1000.0];
    for &inches in &original_values {
        let meters = convert_inches_to_meters(inches).unwrap();
        // Convert back: meters * 39.37 should give us original inches
        let round_trip = meters * 39.37;
        // Allow for small floating point differences
        assert!(
            (round_trip - inches).abs() < 0.0001,
            "Round trip failed for {}: got {}, expected ~{}",
            inches,
            round_trip,
            inches
        );
    }
}
