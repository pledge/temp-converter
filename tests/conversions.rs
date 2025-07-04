use temp_converter::{Temperature, Scale};

const ABSOLUTE_ZERO_C: f64 = -273.15;
const WATER_FREEZING_C: f64 = 0.0;
const WATER_BOILING_C: f64 = 100.0;

fn assert_approx_eq(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{} is not approximately equal to {}", a, b);
}

#[test]
fn test_celsius_to_fahrenheit() {
    let temp = Temperature::new(WATER_FREEZING_C, Scale::Celsius);
    assert_approx_eq(temp.to_fahrenheit().value, 32.0);

    let temp = Temperature::new(WATER_BOILING_C, Scale::Celsius);
    assert_approx_eq(temp.to_fahrenheit().value, 212.0);
}

#[test]
fn test_celsius_to_kelvin() {
    let temp = Temperature::new(WATER_FREEZING_C, Scale::Celsius);
    assert_approx_eq(temp.to_kelvin().value, 273.15);

    let temp = Temperature::new(ABSOLUTE_ZERO_C, Scale::Celsius);
    assert_approx_eq(temp.to_kelvin().value, 0.0);
}

#[test]
fn test_celsius_to_rankine() {
    let temp = Temperature::new(WATER_FREEZING_C, Scale::Celsius);
    assert_approx_eq(temp.to_rankine().value, 491.67);
}

#[test]
fn test_fahrenheit_to_celsius() {
    let temp = Temperature::new(32.0, Scale::Fahrenheit);
    assert_approx_eq(temp.to_celsius().value, WATER_FREEZING_C);

    let temp = Temperature::new(212.0, Scale::Fahrenheit);
    assert_approx_eq(temp.to_celsius().value, WATER_BOILING_C);
}

#[test]
fn test_kelvin_to_celsius() {
    let temp = Temperature::new(273.15, Scale::Kelvin);
    assert_approx_eq(temp.to_celsius().value, WATER_FREEZING_C);

    let temp = Temperature::new(0.0, Scale::Kelvin);
    assert_approx_eq(temp.to_celsius().value, ABSOLUTE_ZERO_C);
}

#[test]
fn test_rankine_to_celsius() {
    let temp = Temperature::new(491.67, Scale::Rankine);
    assert_approx_eq(temp.to_celsius().value, WATER_FREEZING_C);
}

#[test]
fn test_scale_from_str() {
    assert_eq!("celsius".parse::<Scale>().unwrap(), Scale::Celsius);
    assert_eq!("f".parse::<Scale>().unwrap(), Scale::Fahrenheit);
    assert!("invalid".parse::<Scale>().is_err());
}
