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
mod test {
  use self::super::super::test_suits::get_test_cases;

  #[test]
  fn sort() {
    let cases = get_test_cases();
    cases.iter().for_each(|case| {
      let problem = &case.case;
      let mut predicted = problem.clone();
      predicted.sort();
      assert_eq!(case.answer, predicted);
    });
  }
}
