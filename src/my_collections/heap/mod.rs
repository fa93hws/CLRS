// TODO change heap to object

pub fn build_heap(arr: &mut Vec<i32>) {
  let heap_size = arr.len();
  if heap_size < 1 { return; }
  for i in (0..=heap_size / 2).rev() { max_heapify(arr, i, heap_size) }
}

/// idx start from 0
pub fn max_heapify(arr: &mut Vec<i32>, idx: usize, heap_size: usize) {
  let left_idx = find_left_child(idx + 1) - 1;
  let right_idx = find_right_child(idx + 1) - 1;
  let mut largest_idx = idx;
  if left_idx < heap_size && arr[left_idx] > arr[largest_idx] { largest_idx = left_idx }
  if right_idx < heap_size && arr[right_idx] > arr[largest_idx] { largest_idx = right_idx }
  if largest_idx != idx {
    arr.swap(idx, largest_idx);
    max_heapify(arr, largest_idx, heap_size)
  }
}

/// argument and return value start from 1
fn find_parent(idx: usize) -> usize { idx >> 1 }

fn find_left_child(idx: usize) -> usize { idx << 1 }

fn find_right_child(idx: usize) -> usize { (idx << 1) + 1 }

#[cfg(test)]
mod test {
  #[test]
  fn find_parent() {
    assert_eq!(super::find_parent(4), 2);
    assert_eq!(super::find_parent(5), 2);
  }

  #[test]
  fn find_left_child() {
    assert_eq!(super::find_left_child(1), 2);
  }

  #[test]
  fn find_right_child() {
    assert_eq!(super::find_right_child(1), 3);
  }

  #[test]
  fn max_heapify() {
    let mut case0 = vec![1, 2, 3];
    let mut case1 = vec![1, 3, 2];
    super::max_heapify(&mut case0, 0, 3);
    super::max_heapify(&mut case1, 0, 3);
    assert_eq!(vec![3, 2, 1], case0);
    assert_eq!(vec![3, 1, 2], case1);
  }

  #[test]
  fn build_heap() {
    let mut case0: Vec<i32> = vec![];
    let mut case1: Vec<i32> = vec![1];
    let mut case2: Vec<i32> = vec![6, 1, 7, 12, 3, 7, 2, 4, 7, -11];
    super::build_heap(&mut case0);
    super::build_heap(&mut case1);
    super::build_heap(&mut case2);
    assert_eq!(vec![] as Vec<i32>, case0);
    assert_eq!(vec![1], case1);
    assert_eq!(vec![12, 7, 7, 6, 3, 7, 2, 4, 1, -11], case2);
  }
}