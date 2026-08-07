use prime_tasks::counting::{count_primes_parallel, count_primes_sequential};
use prime_tasks::data::load_numbers;
use std::time::Instant;

const DATA_FILE: &str = "data/numbers.txt";

/// Number of worker threads to use: as many as the machine has cores.
fn default_threads() -> usize {
  std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(1)
}

fn run() {
  let numbers = load_numbers(DATA_FILE).expect("failed to read data file");
  let threads = default_threads();
  let count = count_primes_parallel(&numbers, threads);

  println!("Loaded {} numbers from {}", numbers.len(), DATA_FILE);
  println!("Worker threads: {}", threads);
  println!("Primes found  : {}", count);
}

fn run_bench() {
  let numbers = load_numbers(DATA_FILE).expect("failed to read data file");
  let threads = default_threads();

  eprintln!("Numbers: {}", numbers.len());
  eprintln!("Threads: {}", threads);
  eprintln!("---");

  let start = Instant::now();
  let seq = count_primes_sequential(&numbers);
  let seq_time = start.elapsed();

  let start = Instant::now();
  let par = count_primes_parallel(&numbers, threads);
  let par_time = start.elapsed();

  eprintln!("sequential: {:>6} primes in {:?}", seq, seq_time);
  eprintln!("parallel  : {:>6} primes in {:?}", par, par_time);
  eprintln!("---");
  eprintln!("match  : {}", seq == par);
  if par_time.as_secs_f64() > 0.0 {
    eprintln!(
      "speedup: {:.2}x",
      seq_time.as_secs_f64() / par_time.as_secs_f64()
    );
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.iter().any(|a| a == "bench" || a == "--bench") {
    run_bench();
  } else {
    run();
  }
}
