mod merge_sort;
mod heap_sort;
mod insert_sort;

#[cfg(test)]
mod tests {
  use crate::test_utils;
  use crate::test_utils::test_case::TestCase;

  fn create_cases(num: usize) -> TestCase<Vec<i32>, Vec<i32>> {
    let case = test_utils::creation::create_random_i32_vector(num, -100, 100);
    TestCase::new(case, |arr| {
      let mut res = arr.clone();
      res.sort();
      res
    })
  }

  #[test]
  fn sort_empty() {
    let mut empty: Vec<i32> = vec![];
    super::insert_sort::sort(&mut empty);
    assert_eq!(empty, []);
    super::merge_sort::sort(&mut empty);
    assert_eq!(empty, []);
  }

  #[test]
  fn sort_one() {
    let mut one = vec![1];
    super::insert_sort::sort(&mut one);
    assert_eq!(one, [1]);
    super::merge_sort::sort(&mut one);
    assert_eq!(one, [1]);
  }

  #[test]
  fn insert_sort() {
    let test_case = create_cases(5);
    let mut predicted = test_case.case.clone();
    super::insert_sort::sort(&mut predicted);
    assert_eq!(test_case.answer, predicted);
  }

  #[test]
  fn merge_sort() {
    let test_case = create_cases(5);
    let mut predicted = test_case.case.clone();
    super::merge_sort::sort(&mut predicted);
    assert_eq!(test_case.answer, predicted);
  }
}
