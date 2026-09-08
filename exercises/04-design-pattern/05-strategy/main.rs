// TODO: Define the CompressionStrategy trait
// trait CompressionStrategy {
//     fn compress(&self, data: &str) -> String;
// }

// TODO: Implement NoCompression
// struct NoCompression;
// impl CompressionStrategy for NoCompression {
//     fn compress(&self, data: &str) -> String { ... };
// }

// TODO: Implement RleCompression
// struct RleCompression;
// impl CompressionStrategy for RleCompression {
//     fn compress(&self, data: &str) -> String { ... };
// }

struct Compressor {
    // TODO: Add a field for the strategy
}

// TODO: Implement Compressor::new() and Compressor::compress()
// impl Compressor {
//     fn new(strategy: Box<dyn CompressionStrategy>) -> Self { ... };
//     fn compress(&self, data: &str) -> String { ... };
// }

fn main() {
    // Adapt the main while you develop the algorithm
    let data = "aaabbcccc";
    //let plain = Compressor::new(Box::new(NoCompression));
    //println!("No compression: {}", plain.compress(data));
    //let rle = Compressor::new(Box::new(RleCompression));
    //println!("Compression: {}", rle.compress(data));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_compression() {
        let c = Compressor::new(Box::new(NoCompression));
        assert_eq!(c.compress("hello"), "hello");
    }

    #[test]
    fn test_rle_compression() {
        let c = Compressor::new(Box::new(RleCompression));
        assert_eq!(c.compress("aaabbcccc"), "a3b2c4");
    }

    #[test]
    fn test_rle_single_chars() {
        let c = Compressor::new(Box::new(RleCompression));
        assert_eq!(c.compress("abc"), "a1b1c1");
    }
}
