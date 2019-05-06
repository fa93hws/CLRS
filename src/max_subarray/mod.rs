extern crate rand;

pub fn by_brute(arr: &Vec<i32>) -> i32 {
  let len = arr.len();
  if len < 2 { return 0 }
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

pub fn by_binary_search(arr: &Vec<i32>) -> i32 {
  let len = arr.len();
  if len < 2 { return 0 }
  let changes = transform_to_changes(arr);
  let (_,_, res) = find_max_subarray_rec(&changes, 0, changes.len() - 1);
  res
}

/// # argument
/// change_arr:
/// low, mid, high: index in change_arr (not origin array)
/// low, high are inclusive
///
/// # return
/// tuple (left_idx, right_idx, max_sum), idx is in change_arr (not origin array)
/// left_idx: [left_idx, mid] (inclusive) gives the maximum left sum
/// right_idx: [mid, right_idx] (inclusive) gives the maximum right sum
/// max_sum: max_left_sum + max_right_sum
fn find_max_crossing_subarray(change_arr: &Vec<i32>, low: usize, mid: usize, high: usize) -> (usize, usize, i32) {
  let mut left_max = std::i32::MIN;
  let mut left_max_idx: usize = mid;
  let mut sum = 0;
  for i in (low..=mid).rev() {
    sum += change_arr[i];
    if sum > left_max {
      left_max = sum;
      left_max_idx = i;
    }
  }
  let mut right_max = std::i32::MIN;
  let mut right_max_idx: usize = mid;
  sum = 0;
  for i in (mid+1)..=high {
    sum += change_arr[i];
    if sum > right_max {
      right_max = sum;
      right_max_idx = i;
    }
  }
  return (left_max_idx, right_max_idx, left_max + right_max);
}

/// # arguments
/// change_arr: after transform_to_changes
/// low, high. idx of the change_arr (not origin array), inclusive
///
/// # returns
/// tuple (low_idx, high_idx, sum)
/// low_idx, high_idx: idx of the change_arr (not origin array)
/// sum: arr[high-idx] - arr[low_idx]
fn find_max_subarray_rec(change_arr: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
  if low == high { return (low, high, change_arr[low]); }
  let mid = (low + high) / 2;
  let (left_low, left_high, left_sum) = find_max_subarray_rec(change_arr, low, mid);
  let (right_low, right_high, right_sum) = find_max_subarray_rec(change_arr, mid + 1, high);
  let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(change_arr, low, mid, high);
  if left_sum >= right_sum && left_sum >= cross_sum { return (left_low, left_high, left_sum); }
  else if right_sum >= cross_sum { return (right_low, right_high, right_sum); }
  else { return (cross_low, cross_high, cross_sum); }
}

fn transform_to_changes(arr: &Vec<i32>) -> Vec<i32> {
  let len = arr.len();
  if len < 2 { panic!("length of the arr must be greater than 1"); }
  let mut result = Vec::new();
  for i in 1..len { result.push(arr[i] - arr[i-1]); }
  result
}

#[cfg(test)]
mod test_brute {
  #[test]
  fn empty() {
    assert_eq!(super::by_brute(&vec![]), 0);
    assert_eq!(super::by_brute(&vec![1]), 0);
  }

  #[test]
  fn simple() {
    assert_eq!(super::by_brute(&vec![-5, -1, 2, 5, 1, 0, 10, -20, 5, 12, 5, 4, 9]), 32);
    assert_eq!(super::by_brute(&vec![-5, -1, 2, 5, 1, 0, 10, -20, 5, 1, 5, 4, 9]), 29);
    assert_eq!(super::by_brute(&vec![-5, -1, 2, 5, 1, 0, 10, -2, 5, 1, 5, 4, 9]), 15);
  }
}

#[cfg(test)]
mod test_util {
  #[test]
  fn transform() {
    let input = vec![1,2,3,4,5];
    let output = vec![1,1,1,1];
    assert_eq!(super::transform_to_changes(&input), output);
  }

  #[test]
  fn find_max_crossing_subarray() {
    let sample = vec![-5, -1, 2, 5, 1, 3, -4];
    assert_eq!(super::find_max_crossing_subarray(&sample, 0, 2, 4), (2, 4, 8));
    assert_eq!(super::find_max_crossing_subarray(&sample, 2, 3, 5), (2, 5, 11));
  }

  fn find_max_subarray_rec() {
    let sample = vec![-5, -1, 2, 5, 1, 3, -4];
    assert_eq!(super::find_max_subarray_rec(&sample, 1, 2), (2, 2, 2));
    assert_eq!(super::find_max_subarray_rec(&sample, 3, 6), (3, 5, 9));
  }
}

#[cfg(test)]
mod test {
  use rand::Rng;

  fn create_subarray_cases(num: i32) -> Vec<(Vec<i32>, i32)> {
    let mut result: Vec<(Vec<i32>, i32)> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
      let size = rng.gen_range(100, 1000);
      let case: Vec<i32> = (0..size).map(|_| {
        rng.gen_range(-100, 100)
      }).collect();
      let ans = super::by_brute(&case);
      result.push((case, ans));
    }
    result
  }

  #[test]
  fn empty() {
    assert_eq!(super::by_binary_search(&vec![]), 0);
    assert_eq!(super::by_binary_search(&vec![1]), 0);
  }

  #[test]
  fn complicate() {
    for (case, ans) in create_subarray_cases(10) {
      assert_eq!(super::by_binary_search(&case), ans);
    }
  }

  #[test]
  fn profile_5k_binary() {
    let mut rng = rand::thread_rng();
    let case: Vec<i32> = (0..=5000).map(|_| {
      rng.gen_range(-10000, 10000)
    }).collect();
    super::by_binary_search(&case);
  }

  #[test]
  fn profile_5k_brute() {
    let mut rng = rand::thread_rng();
    let case: Vec<i32> = (0..=5000).map(|_| {
      rng.gen_range(-10000, 10000)
    }).collect();
    super::by_brute(&case);
  }
}