mod sort;
mod max_subarray;

fn main() {
  let mut arr = vec![6,1,7,12,3,7,2,4,7,-11];
  sort::heap_sort(&mut arr);
}
