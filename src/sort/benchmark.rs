use crate::test_utils::creation::create_random_i32_vector;
use crate::time::PreciseTime;
use crate::sort::{heap_sort, insert_sort, merge_sort};

enum SortType {
  MergeSort,
  HeapSort,
  InsertSort,
}

fn bench(mode: SortType, bench_size: usize, array_size: usize) -> i64 {
  let mut duration: i64 = 0;
  for _ in 0..bench_size {
    let mut problem = create_random_i32_vector(array_size, -10000, 10000);
    let start = PreciseTime::now();
    match mode {
      SortType::MergeSort => merge_sort::sort(&mut problem),
      SortType::HeapSort => heap_sort::sort(&mut problem),
      SortType::InsertSort => insert_sort::sort(&mut problem),
    }
    let end = PreciseTime::now();
    duration += start.to(end).num_nanoseconds().unwrap() / 1000;
  }
  duration / bench_size as i64
}

pub fn run(bench_size: usize, array_size: usize) {
  println!("benchmarking max_subarray ...");
  let heap = bench(SortType::HeapSort, bench_size, array_size);
  let merge = bench(SortType::MergeSort, bench_size, array_size);
  let insert = bench(SortType::InsertSort, bench_size, array_size);
  println!("heap sort on {} array {} times ... {} kns", array_size, bench_size, heap);
  println!("merge sort on {} array {} times ... {} kns", array_size, bench_size, merge);
  println!("insert sort on {} array {} times ... {} kns", array_size, bench_size, insert);
}