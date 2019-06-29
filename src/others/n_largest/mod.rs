use std::collections::BinaryHeap;

pub fn find_n_smallest_in_vec(vec: &Vec<i32>, n: usize) -> Vec<i32> {
  let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(n);
  vec.iter().for_each(|num| {
    if heap.len() == n && heap.peek().unwrap() > num {
      heap.pop();
    }
    if heap.len() < n {
      heap.push(*num);
    }
  });
  let mut result = heap.into_vec();
  result.sort_unstable();
  result
}

#[cfg(test)]
mod tests {
  #[test]
  fn empty_array() {
    let array: Vec<i32> = vec![];
    let num = 1;
    let received = super::find_n_smallest_in_vec(&array, num);
    let expected: Vec<i32> = vec![];
    assert_eq!(expected, received);
  }

  #[test]
  fn n_larger_than_array_size() {
    let array = vec![3, 5, 2, 1, 5];
    let num = 999;
    let received = super::find_n_smallest_in_vec(&array, num);
    let mut expected: Vec<i32> = array.clone();
    expected.sort_unstable();
    assert_eq!(expected, received);
  }

  #[test]
  fn n_smallest_in_array() {
    let array = vec![3, 1, -6, 1, 2, 5, 7, 1, 0, 0, -2, 0, 8, 4];
    let num = 3;
    let received = super::find_n_smallest_in_vec(&array, num);
    let expected = vec![-6, -2, 0];
    assert_eq!(expected, received);
  }
}
