// Weather-satellite telemetry helper.
//
// This file does NOT compile: it contains four type errors.
// Read the compiler output, fix the errors, keep the intent of the code.

const MAX_CHANNELS: u8 = 300;

const DUTY_CYCLE: f64 = 1 / 2;

fn mean_temperature(reading_a: f64, reading_b: i32) -> f64 {
    (reading_a + reading_b) / 2.0
}

// This function is fine - do not change it.
fn symbol_for(unit: char) -> &'static str {
    match unit {
        'C' => "celsius",
        'F' => "fahrenheit",
        _ => "unknown",
    }
}

fn main() {
    println!("max channels: {MAX_CHANNELS}");
    println!("duty cycle: {:.0}%", DUTY_CYCLE * 100.0);
    println!("mean temperature: {:.2}", mean_temperature(20.5, 22));
    println!("unit 'C' is: {}", symbol_for("C"));

    let packet_channels = 64;
    println!("packet uses {packet_channels} of {MAX_CHANNELS} channels");
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_channels_type() {
        // 300 fits in u16 (0..=65_535) but not in u8 (0..=255).
        let _: u16 = MAX_CHANNELS;
        assert_eq!(MAX_CHANNELS, 300);
    }

    #[test]
    fn test_duty_cycle() {
        assert_eq!(DUTY_CYCLE, 0.5);
    }

    #[test]
    fn test_mean_temperature() {
        // (20.5 + 22.0) / 2.0 - the i32 reading must not be truncated.
        assert_eq!(mean_temperature(20.5, 22), 21.25);
        assert_eq!(mean_temperature(21.0, 25), 23.0);
    }

    #[test]
    fn test_symbol_for() {
        assert_eq!(symbol_for('C'), "celsius");
        assert_eq!(symbol_for('F'), "fahrenheit");
        assert_eq!(symbol_for('K'), "unknown");
    }
}
