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
  use self::super::super::test_suits::CASES;

  #[test]
  fn sort() {
    CASES.iter().for_each(|case| {
      let problem = &case.case;
      assert_eq!(case.answer, super::find(problem));
    });
  }
}
