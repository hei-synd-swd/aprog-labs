// TODO: define the `TrafficLight` enum with variants `Red`, `Yellow`, `Green`.

// TODO: implement `next` — transition to the next state.
// fn next(light: &TrafficLight) -> TrafficLight { ... }

// TODO: implement `is_stop` — return true only for Red.
// fn is_stop(light: &TrafficLight) -> bool { ... }

fn main() {
    let mut light = TrafficLight::Red;
    println!("red, stop? {}", is_stop(&light));
    light = next(&light);
    println!("green, stop? {}", is_stop(&light));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_to_green() {
        let light = TrafficLight::Red;
        assert!(matches!(next(&light), TrafficLight::Green));
    }

    #[test]
    fn test_green_to_yellow() {
        let light = TrafficLight::Green;
        assert!(matches!(next(&light), TrafficLight::Yellow));
    }

    #[test]
    fn test_yellow_to_red() {
        let light = TrafficLight::Yellow;
        assert!(matches!(next(&light), TrafficLight::Red));
    }

    #[test]
    fn test_is_stop() {
        assert!(is_stop(&TrafficLight::Red));
        assert!(!is_stop(&TrafficLight::Yellow));
        assert!(!is_stop(&TrafficLight::Green));
    }
}
