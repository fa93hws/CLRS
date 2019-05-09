extern crate rand;
use rand::Rng;

pub fn create_random_i32_vector(size: usize, low: i32, high: i32) -> Vec<i32> {
  let mut result: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
  let mut rng = rand::thread_rng();
  (0..size).map(|_| {
    rng.gen_range(low, high)
  }).collect()
}

#[cfg(test)]
mod test {
  #[test]
  fn create_random_i32_vector() {
    let (size, low, high) = (100, -1, 1);
    let res = super::create_random_i32_vector(size, low, high);
    assert_eq!(100, res.len());
    res.iter().for_each(|ele| {
      assert_eq!(true, *ele >= -1 && *ele <= 1);
    })
  }
}