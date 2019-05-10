pub fn sort(arr: &mut Vec<i32>) {
  for i in 0..arr.len() {
    for j in (0..i).rev() {
      match arr {
        _ if arr[j] > arr[j + 1] => arr.swap(j, j + 1),
        _ => break,
      }
    }
  }
}

#[cfg(test)]
mod tests {
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
