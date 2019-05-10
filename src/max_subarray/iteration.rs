pub fn find(arr: &Vec<i32>) -> i32 {
  let len = arr.len();
  if len < 2 { return 0; }
  let mut min = std::i32::MAX;
  let mut sum = 0;
  for i in 0..len {
    if arr[i] < min { min = arr[i]; }
    if arr[i] - min > sum { sum = arr[i] - min }
  }
  sum
}

#[cfg(test)]
mod test {
  #[test]
  fn empty() {
    assert_eq!(super::find(&vec![]), 0);
    assert_eq!(super::find(&vec![1]), 0);
  }

  #[test]
  fn simple() {
    assert_eq!(super::find(&vec![-5, -1, 2, 5, 1, 0, 10, -20, 5, 12, 5, 4, 9]), 32);
    assert_eq!(super::find(&vec![-5, -1, 2, 5, 1, 0, 10, -20, 5, 1, 5, 4, 9]), 29);
    assert_eq!(super::find(&vec![-5, -1, 2, 5, 1, 0, 10, -2, 5, 1, 5, 4, 9]), 15);
  }
}
