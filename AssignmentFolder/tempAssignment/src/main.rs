const freezingPoint: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - freezingPoint) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + freezingPoint
}

fn main() {
    let mut temperatureFahrenheit: f64 = 32.0;
    let temperatureCelsius = fahrenheit_to_celsius(temperatureFahrenheit);

    for _ in 0..5{
        temperatureFahrenheit += 1.0;
        let temperatureCelsius = fahrenheit_to_celsius(temperatureFahrenheit);
        println!("{}°F is equal to {:.2}°C", temperatureFahrenheit, temperatureCelsius);
    }
}
