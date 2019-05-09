pub struct TestCase<T, R> {
  pub case: T,
  pub answer: R,
}

impl<T, R> TestCase<T, R> {
  pub fn new<F>(case: T, func: F) -> TestCase<T, R>
    where F: Fn(&T) -> R {
    let answer = func(&case);
    TestCase { case, answer }
  }
}

#[cfg(test)]
mod test {
  fn double(arr: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    arr.iter().for_each(|a| { res.push(*a * 2) });
    res
  }

  #[test]
  fn new() {
    let test_case0: super::TestCase<i32, i32> = super::TestCase::new(1, |int| {
      *int + 1
    });
    assert_eq!(test_case0.case, 1);
    assert_eq!(test_case0.answer, 2);
    let test_case1: super::TestCase<Vec<i32>, Vec<i32>> = super::TestCase::new(vec![1, 2, 3], double);
    assert_eq!(test_case1.case, vec![1, 2, 3]);
    assert_eq!(test_case1.answer, vec![2, 4, 6]);
  }
}
