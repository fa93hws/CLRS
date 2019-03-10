mod sort;

fn main() {
    let mut arr = [3, 1, 6, 3, 6, 10, -1, 4];
    sort::merge_sort(&mut arr);
    println!("{:?}", arr);
}
