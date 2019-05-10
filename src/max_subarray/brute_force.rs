pub fn find(arr: &Vec<i32>) -> i32 {
  let len = arr.len();
  if len < 2 { return 0; }
  let mut max_diff = 0;
  let mut diff: i32;
  for i in 0..(len - 1) {
    for j in (i + 1)..len {
      diff = arr[j] - arr[i];
      if diff > max_diff { max_diff = diff; }
    }
  }
  max_diff
}

#[cfg(test)]
mod tests {
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
