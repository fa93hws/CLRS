#[macro_use]
extern crate lazy_static;
extern crate time;

mod sort;
mod max_subarray;
mod my_collections;
mod test_utils;

const BENCH_SAMPLE_SIZE: usize = 50;

fn main() {
  println!();
  sort::benchmark::run(BENCH_SAMPLE_SIZE, 500);
  println!();
  max_subarray::benchmark::run(BENCH_SAMPLE_SIZE, 500);
}

