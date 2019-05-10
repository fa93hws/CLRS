use crate::test_utils::creation::create_random_i32_vector;
use crate::time::PreciseTime;
use crate::max_subarray::{brute_force, binary_search, iteration};

enum FindType {
  Brute,
  Binary,
  Iteration,
}

fn bench(mode: FindType, bench_size: usize, array_size: usize) -> i64 {
  let problem = create_random_i32_vector(array_size, -10000, 10000);
  let start = PreciseTime::now();
  for _ in 0..bench_size {
    match mode {
      FindType::Brute => brute_force::find(&problem),
      FindType::Binary => binary_search::find(&problem),
      FindType::Iteration => iteration::find(&problem),
    };
  }
  let end = PreciseTime::now();
  return start.to(end).num_nanoseconds().unwrap() / 1000 / bench_size as i64;
}

pub fn run(bench_size: usize, array_size: usize) {
  println!("benchmarking max_subarray ...");
  let brute = bench(FindType::Brute, bench_size, array_size);
  let binary = bench(FindType::Binary, bench_size, array_size);
  let iteration = bench(FindType::Iteration, bench_size, array_size);
  println!("brute sort on {} array {} times ... {} kns", array_size, bench_size, brute);
  println!("binary sort on {} array {} times ... {} kns", array_size, bench_size, binary);
  println!("iteration sort on {} array {} times ... {} kns", array_size, bench_size, iteration);
}
