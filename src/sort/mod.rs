extern crate rand;
mod merge_sort;

pub fn insert_sort<T: Ord + Copy>(arr: &mut [T]) {
  for i in 0..arr.len() {
    for j in (0..i).rev() {
      match arr {
        _ if arr[j] > arr[j + 1] => arr.swap(j, j + 1),
        _ => break,
      }
    }
  }
}

pub fn merge_sort(arr: &mut [i32]) {
  merge_sort::sort(arr);
}

#[cfg(test)]
mod tests {
  use rand::Rng;

  fn create_cases(num: i32) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut result: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
      let size = rng.gen_range(0, 5000);
      let case: Vec<i32> = (0..size).map(|_| {
        rng.gen::<i32>()
      }).collect();
      let mut ans = case.clone();
      ans.sort();
      result.push((case, ans));
    }
    result
  }

  #[test]
  fn sort_empty() {
    let mut empty:[i32; 0] = [];
    super::insert_sort(&mut empty);
    assert_eq!(empty, []);
    super::merge_sort(&mut empty);
    assert_eq!(empty, []);
  }

  #[test]
  fn sort_one() {
    let mut one = [1];
    super::insert_sort(&mut one);
    assert_eq!(one, [1]);
    super::merge_sort(&mut one);
    assert_eq!(one, [1]);
  }

  #[test]
  fn insert_sort() {
    for (mut case, ans) in create_cases(10) {
      super::insert_sort(&mut case);
      assert_eq!(case, ans);
    }
  }

  #[test]
  fn merge_sort() {
    for (mut case, ans) in create_cases(10) {
      super::merge_sort(&mut case);
      assert_eq!(case, ans);
    }
  }
}
