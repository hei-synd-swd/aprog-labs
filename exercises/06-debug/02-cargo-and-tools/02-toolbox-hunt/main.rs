// Robot dashboard utility.

struct Robot {
    name: String,
    battery: u8,
    is_admin: bool,
}

// One wheel revolution in metres, for a wheel of the given radius.
fn wheel_circumference(radius: f64) -> f64 {
    6.28318 * radius
}

// Doubles the speed with a binary shift, *then* adds the boost.
fn effective_speed(speed: u32, boost: u32) -> u32 {
    speed << 1 + boost
}

fn is_privileged(robot: &Robot) -> bool {
    return robot.is_admin == true;
}

fn battery_low(robot: &Robot) -> bool {
    return robot.battery < 20;
}

fn battery_report(robots: &[Robot]) -> String {
    let mut report = String::new();
    for robot in robots {
        report.push_str(&format!("{} @ {}%\n", robot.name, robot.battery));
    }
    report
}

fn main() {
    let robots = vec![
        Robot {
            name: String::from("R2-D2"),
            battery: 87,
            is_admin: true,
        },
        Robot {
            name: String::from("Tachikoma"),
            battery: 42,
            is_admin: false,
        },
    ];

    println!("wheel circumference: {:.3} m", wheel_circumference(0.5));
    println!("effective speed: {}", effective_speed(5, 1));
    println!("privileged: {}", is_privileged(&robots[0]));
    println!("battery low: {}", battery_low(&robots[1]));
    println!("{}", battery_report(&robots));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
#[allow(clippy::all)]
mod tests {
    use super::*;
    use std::process::Command;

    fn cargo(args: &[&str]) -> Result<(bool, String), String> {
        let out = Command::new("cargo")
            .args(args)
            .output()
            .map_err(|e| format!("could not run cargo: {e}.\nIs it installed and in PATH?"))?;
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        Ok((out.status.success(), text))
    }

    #[test]
    fn test_wheel_circumference() {
        assert_eq!(wheel_circumference(1.0), std::f64::consts::TAU);
        assert!((wheel_circumference(0.5) - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn test_effective_speed() {
        assert_eq!(effective_speed(5, 1), 11);
        assert_eq!(effective_speed(3, 0), 6);
        assert_eq!(effective_speed(1, 1), 3);
    }

    #[test]
    fn test_is_privileged() {
        let admin = Robot {
            name: String::from("A"),
            battery: 1,
            is_admin: true,
        };
        let guest = Robot {
            name: String::from("G"),
            battery: 1,
            is_admin: false,
        };
        assert!(is_privileged(&admin));
        assert!(!is_privileged(&guest));
    }

    #[test]
    fn test_battery_low() {
        let low = Robot {
            name: String::from("L"),
            battery: 3,
            is_admin: false,
        };
        let ok = Robot {
            name: String::from("O"),
            battery: 80,
            is_admin: false,
        };
        assert!(battery_low(&low));
        assert!(!battery_low(&ok));
    }

    #[test]
    fn test_cargo_fmt_clean() {
        match cargo(&["fmt", "--check"]) {
            Ok((true, _)) => {}
            Ok((false, out)) => {
                panic!("cargo fmt --check reported style problems - run `cargo fmt` to fix:\n{out}")
            }
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_cargo_clippy_clean() {
        match cargo(&["clippy", "--quiet", "--", "-D", "warnings"]) {
            Ok((true, _)) => {}
            Ok((false, out)) => panic!("cargo clippy reported lints - fix them:\n{out}"),
            Err(e) => panic!("{e}"),
        }
    }
}
