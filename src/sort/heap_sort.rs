use my_collections::heap;

/// build the max heap
pub fn sort(arr: &mut Vec<i32>) {
  heap::build_heap(arr);
  let mut heap_size = arr.len();
  for i in (1..arr.len()).rev() {
    arr.swap(0, i);
    heap_size -= 1;
    heap::max_heapify(arr, 0, heap_size);
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn heap_sort() {
    let mut case0: Vec<i32> = vec![];
    let mut case1: Vec<i32> = vec![1];
    let mut case2: Vec<i32> = vec![6, 1, 7, 12, 3, 7, 2, 4, 7, -11];
    super::sort(&mut case0);
    super::sort(&mut case1);
    super::sort(&mut case2);
    assert_eq!(vec![] as Vec<i32>, case0);
    assert_eq!(vec![1], case1);
    assert_eq!(vec![-11, 1, 2, 3, 4, 6, 7, 7, 7, 12], case2);
  }
}
