pub fn sort(arr: &mut Vec<i32>) {
  sort_rec(arr, 0, arr.len() - 1);
}

fn sort_rec(arr: &mut Vec<i32>, begin: usize, end: usize) {
  if begin < end {
    let pivot = partition(arr, begin, end, (begin + end) / 2);
    sort_rec(arr, begin, pivot);
    sort_rec(arr, pivot + 1, end);
  }
}

fn partition(arr: &mut Vec<i32>, begin: usize, end: usize, pivot: usize) -> usize {
  let x = arr[pivot];
  let mut i = begin;
  for j in begin..=end {
    if arr[j] < x {
      arr.swap(i, j);
      i += 1;
    }
  }
  arr.swap(i, pivot);
  i
}

#[cfg(test)]
mod tests {
  use self::super::super::test_suits::CASES;

  #[test]
  fn partition() {
    let mut case0 = vec![4, 2, 1, 4, 2, 9, 1, -2, 5, 4, 2, 7, 2];
    let par0 = super::partition(&mut case0, 0, 7, 4);
    assert_eq!(vec![1, 1, -2, 2, 4, 9, 2, 4, 5, 4, 2, 7, 2], case0);
    assert_eq!(3, par0);
    let mut case1 = vec![4, 2, 1];
    let par1 = super::partition(&mut case1, 0, 2, 1);
    assert_eq!(vec![1, 2, 4], case1);
    assert_eq!(1, par1);
  }

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
