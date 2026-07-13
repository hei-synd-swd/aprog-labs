// TODO: 16711935 written as a hex literal (prefix 0x) with `_` separators.
const COLOR_MAGENTA: u32 = 0; // TODO

// TODO: 493 written as an octal literal (prefix 0o).
const UNIX_PERMS: u32 = 0; // TODO

// TODO: 255 written as a binary literal with all 8 bits set (prefix 0b).
const ALL_FLAGS: u8 = 0; // TODO

// TODO: the ASCII code of 'A' written as a byte literal (b'...').
const LETTER_A: u8 = 0; // TODO

fn main() {
    println!("{COLOR_MAGENTA} {UNIX_PERMS} {ALL_FLAGS} {LETTER_A}");
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_magenta() {
        assert_eq!(COLOR_MAGENTA, 0xFF_00_FF);
    }

    #[test]
    fn test_unix_perms() {
        assert_eq!(UNIX_PERMS, 493);
    }

    #[test]
    fn test_all_flags() {
        assert_eq!(ALL_FLAGS, 255);
    }

    #[test]
    fn test_letter_a() {
        assert_eq!(LETTER_A, 65);
    }
}
