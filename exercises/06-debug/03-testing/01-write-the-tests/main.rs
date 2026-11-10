// Quality-check helpers for a weather station

fn grade(score: u32) -> &'static str {
    assert!(score <= 100, "score must be 0..=100, got {score}");
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn parse_temperature(raw: &str) -> Option<i32> {
    let digits = raw.strip_suffix('C')?;
    let negative = digits.starts_with('-');
    let body = if negative { &digits[1..] } else { digits };
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let value: i32 = body.parse().ok()?;
    Some(if negative { -value } else { value })
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn main() {
    println!("grade(85) = {}", grade(85));
    println!(
        "parse_temperature(\"-3C\") = {:?}",
        parse_temperature("-3C")
    );
    println!("celsius_to_fahrenheit(37) = {}", celsius_to_fahrenheit(37));
}

//==============================================================================
//                       STUDENT TESTS - ADD YOURS HERE
//   Write at least 6 #[test] functions. Do not move or rename this module.
//==============================================================================
#[cfg(test)]
mod student_tests {
    use super::*;

    // #[test]
    // fn grade_90_is_an_a() {
    //     assert_eq!(grade(90), "A");
    // }

    // TODO: add your tests above this line.
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_functions_are_intact() {
        assert_eq!(grade(0), "F");
        assert_eq!(grade(95), "A");
        assert_eq!(parse_temperature("21C"), Some(21));
        assert_eq!(parse_temperature("-3C"), Some(-3));
        assert_eq!(parse_temperature("oops"), None);
        assert_eq!(celsius_to_fahrenheit(100), 212);
        assert_eq!(celsius_to_fahrenheit(-40), -40);
    }
}
