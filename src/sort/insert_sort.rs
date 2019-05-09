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