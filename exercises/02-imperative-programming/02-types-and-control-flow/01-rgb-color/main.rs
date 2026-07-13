// TODO: define the `Rgb` struct with fields `r`, `g`, `b` (all `u8`).

// TODO: implement `to_hex` that returns a `String` like "#FF0000".
// fn to_hex(rgb: &Rgb) -> String { ... }

// TODO: define constants WHITE, BLACK, RED, GREEN, BLUE

fn main() {
    let orange = Rgb {
        r: 255,
        g: 128,
        b: 0,
    };
    println!("orange = {}", to_hex(&orange));
    println!("white  = {}", to_hex(&WHITE));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(&Rgb { r: 255, g: 0, b: 0 }), "#FF0000");
        assert_eq!(to_hex(&Rgb { r: 0, g: 255, b: 0 }), "#00FF00");
        assert_eq!(to_hex(&Rgb { r: 0, g: 0, b: 255 }), "#0000FF");
        assert_eq!(
            to_hex(&Rgb {
                r: 17,
                g: 34,
                b: 51
            }),
            "#112233"
        );
    }

    #[test]
    fn test_constants() {
        assert_eq!(to_hex(&WHITE), "#FFFFFF");
        assert_eq!(to_hex(&BLACK), "#000000");
        assert_eq!(to_hex(&RED), "#FF0000");
        assert_eq!(to_hex(&GREEN), "#00FF00");
        assert_eq!(to_hex(&BLUE), "#0000FF");
    }

    #[test]
    fn test_fields() {
        let c = Rgb {
            r: 10,
            g: 20,
            b: 30,
        };
        assert_eq!(c.r, 10);
        assert_eq!(c.g, 20);
        assert_eq!(c.b, 30);
    }
}
