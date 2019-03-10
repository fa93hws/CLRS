// beg...(split - 1) | split..end + 1
fn merge<T: Ord + Copy>(arr: &mut [T], beg: usize, end: usize, split: usize) {
    if beg == split - 1 || split == end {
        return;
    }
    let mut merge_result = Vec::with_capacity(end - beg + 1);
    let mut i = beg;
    let mut j = split;
    while i < split && j < end + 1 {
        if j == end + 1 || arr[i] < arr[j] {
            merge_result.push(arr[i]);
            i += 1;
        } else {
            merge_result.push(arr[j]);
            j += 1;
        }
    }
    for idx in 0..merge_result.len() {
        arr[beg + idx] = merge_result[idx];
    }
}

fn split<T: Ord + Copy>(arr: &mut [T], beg: usize, end: usize) {
    match end - beg {
        0 => (),
        1 if arr[beg] > arr[end] => {
            arr.swap(beg, end);
            merge(arr, beg, end, end);
        },
        1 => (),
        _ => {
            let mid = (beg + end) / 2;
            split(arr, beg, mid);
            split(arr, mid + 1, end);
            merge(arr, beg, end, mid + 1);
        }
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    match arr.len() {
        0 | 1 => return,
        _ => split(arr, 0, arr.len() - 1)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn merge() {
        let mut case0 = [1, 3, 4, 5, 2, 5, 10, 12];
        super::merge(&mut case0, 0, 7, 4);
        assert_eq!(case0, [1, 2, 3, 4, 5, 5, 10, 12]);
    }
}