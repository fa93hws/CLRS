extern crate rand;

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

#[cfg(test)]
mod tests {
    use rand::Rng;

    fn create_cases(num: i32) -> Vec<(Vec<i32>, Vec<i32>)> {
        let mut result: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num {
            let size = rng.gen_range(0, 100);
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
    fn insert_sort() {
        for (mut case, ans) in create_cases(1000) {
            super::insert_sort(&mut case);
            assert_eq!(case, ans);
        }
    }
}
