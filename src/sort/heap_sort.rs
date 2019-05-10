use crate::my_collections::heap::array_heap;

/// build the max heap
pub fn sort(arr: &mut Vec<i32>) {
  array_heap::build_heap(arr);
  let mut heap_size = arr.len();
  for i in (1..arr.len()).rev() {
    arr.swap(0, i);
    heap_size -= 1;
    array_heap::max_heapify(arr, 0, heap_size);
  }
}

#[cfg(test)]
mod test {
  use self::super::super::test_suits::CASES;

  #[test]
  fn sort() {
    CASES.iter().for_each(|case| {
      let problem: &Vec<i32> = &case.problem;
      let mut predicted = problem.clone();
      predicted.sort();
      assert_eq!(case.answer, predicted);
    });
  }
}
