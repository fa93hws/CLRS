use std::sync::Once;
use crate::test_utils::creation::create_random_i32_vector;
use crate::test_utils::test_case::TestCase;

const CASES_SIZE: usize = 12;
static mut CASES: [TestCase<Vec<i32>, Vec<i32>>; CASES_SIZE] = [];
static INIT: Once = Once::new();

fn create_test_problems() -> Vec<Vec<i32>> {
  let mut problems = vec![
    vec![] as Vec<i32>,
    vec![1]
  ];
  for _ in 0..(CASES_SIZE - problems.len()) {
    let random_case = create_random_i32_vector(500, -100, 100);
    problems.push(random_case);
  }
  problems
}

fn create_test_case(problem: Vec<i32>) {
  let test_case = TestCase::new(problem, |p| {
    let mut answer = p.clone();
    answer.sort();
    answer
  });
  unsafe { CASES.push(test_case); }
}

fn create_test_cases() {
  let problems = create_test_problems();
  for problem in problems { create_test_case(problem); }
}

pub fn get_test_cases() -> &'static Vec<TestCase<Vec<i32>, Vec<i32>>> {
  INIT.call_once(create_test_cases);
  unsafe { &CASES }
}
