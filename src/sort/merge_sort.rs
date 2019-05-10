// beg...split | split..end + 1
fn merge(arr: &mut Vec<i32>, beg: usize, end: usize, split: usize) {
  let mut merge_result = Vec::with_capacity(end - beg + 1);
  let mut i = beg;
  let mut j = split;
  while i < split || j < end + 1 {
    if j >= end + 1 {
      merge_result.push(arr[i]);
      i += 1;
    } else if i >= split {
      merge_result.push(arr[j]);
      j += 1;
    } else if arr[i] > arr[j] {
      merge_result.push(arr[j]);
      j += 1;
    } else {
      merge_result.push(arr[i]);
      i += 1;
    }
  }
  for idx in 0..merge_result.len() {
    arr[beg + idx] = merge_result[idx];
  }
}

fn split(arr: &mut Vec<i32>, beg: usize, end: usize) {
  match end - beg {
    0 => (),
    _ => {
      let mid = (beg + end) / 2;
      split(arr, beg, mid);
      split(arr, mid + 1, end);
      merge(arr, beg, end, mid + 1);
    }
  }
}

pub fn sort(arr: &mut Vec<i32>) {
  match arr.len() {
    0 | 1 => return,
    _ => split(arr, 0, arr.len() - 1)
  }
}

#[cfg(test)]
mod test {
  use self::super::super::test_suits::CASES;

  #[test]
  fn sort() {
    CASES.iter().for_each(|case| {
      let problem = &case.case;
      let mut predicted = problem.clone();
      predicted.sort();
      assert_eq!(case.answer, predicted);
    });
  }
}
