use crate::test_utils::creation::create_random_i32_vector;
use crate::test_utils::test_case::TestCase;

const CASES_SIZE: usize = 12;

lazy_static! {
  static ref CASES: Vec<TestCase<Vec<i32>, Vec<i32>>> = {
    create_test_cases()
  };
}

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

fn create_test_case(problem: &Vec<i32>) -> TestCase<Vec<i32>, Vec<i32>> {
  TestCase::new(problem.clone(), |p| {
    let mut answer = p.clone();
    answer.sort();
    answer
  })
}

fn create_test_cases() -> Vec<TestCase<Vec<i32>, Vec<i32>>> {
  let problems = create_test_problems();
  problems.iter().map(create_test_case).collect()
}

pub fn get_test_cases() -> &'static Vec<TestCase<Vec<i32>, Vec<i32>>> {
  &CASES
}
