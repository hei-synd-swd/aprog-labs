fn main() {
  #[cfg(all(feature = "part1", feature = "part2"))]
  compile_error!("Can not enable part1 and part2 tests at the same time!");
}
