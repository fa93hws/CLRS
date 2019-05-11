pub struct MaxHeap {
  array: Vec<i32>
}

fn panic_if_empty(arr: &Vec<i32>) {
  if arr.is_empty() {
    panic!("heap underflow");
  }
}

impl MaxHeap {
  pub fn new(arr: &Vec<i32>) -> Self {
    let mut array = arr.clone();
    build_heap(&mut array);
    MaxHeap { array }
  }
}

impl MaxHeap {
  pub fn size(&self) -> usize { self.array.len() }

  pub fn get_max(&self) -> i32 {
    panic_if_empty(&self.array);
    self.array[0]
  }

  pub fn extract_max(&mut self) -> i32 {
    panic_if_empty(&self.array);
    let max = self.array.swap_remove(0);
    let size = self.size();
    max_heapify(&mut self.array, 0, size);
    max
  }

  pub fn increase_key(&mut self, idx: usize, value: i32) {
    if idx >= self.size() {
      panic!("heap underflow");
    } else if self.array[idx] > value {
      panic!("can only increase the key");
    }
    self.array[idx] = value;
    let mut current_idx = idx;
    loop {
      if current_idx == 0 { break; }
      let parent_idx = find_parent(current_idx + 1) - 1;
      if self.array[parent_idx] > self.array[current_idx] { break; }
      self.array.swap(parent_idx, current_idx);
      current_idx = parent_idx;
    }
  }

  pub fn insert(&mut self, value: i32) {
    self.array.push(std::i32::MIN);
    self.increase_key(self.size() - 1, value);
  }
}


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
fn find_parent(idx: usize) -> usize {
  idx >> 1
}

fn find_left_child(idx: usize) -> usize { idx << 1 }

fn find_right_child(idx: usize) -> usize { (idx << 1) + 1 }

#[cfg(test)]
mod test {
  use crate::my_collections::heap::array_heap::MaxHeap;
  use crate::sort::test_suits::CASES as sort_test_cases;

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
  fn heap_new() {
    let case0: Vec<i32> = vec![];
    let case1: Vec<i32> = vec![1];
    let case2: Vec<i32> = vec![6, 1, 7, 12, 3, 7, 2, 4, 7, -11];
    let heap0 = MaxHeap::new(&case0);
    let heap1 = MaxHeap::new(&case1);
    let heap2 = MaxHeap::new(&case2);
    assert_eq!(0, heap0.size());
    assert_eq!(1, heap1.size());
    assert_eq!(vec![12, 7, 7, 6, 3, 7, 2, 4, 1, -11], heap2.array);
  }

  #[test]
  #[should_panic]
  fn get_max_from_empty() {
    let heap = MaxHeap::new(&vec![]);
    heap.get_max();
  }

  #[test]
  #[should_panic]
  fn extract_max_from_empty() {
    let mut heap = MaxHeap::new(&vec![]);
    heap.extract_max();
  }

  #[test]
  fn extract_max() {
    sort_test_cases.iter().for_each(|case| {
      let mut heap: MaxHeap = MaxHeap::new(&case.problem);
      for i in (0..case.answer.len()).rev() {
        assert_eq!(case.answer[i], heap.extract_max());
      }
    });
  }

  #[test]
  fn increase_key() {
    let mut heap = MaxHeap::new(&vec![6, 1, 7, 12, 3, 7, 2, 4, 7, -11]);
    // 12, 7, 7, 6, 3, 7, 2, 4, 1, -11
    heap.increase_key(5, 9);
    assert_eq!(vec![12, 7, 9, 6, 3, 7, 2, 4, 1, -11], heap.array);
    heap.increase_key(5, 15);
    assert_eq!(vec![15, 7, 12, 6, 3, 9, 2, 4, 1, -11], heap.array);
  }
}
