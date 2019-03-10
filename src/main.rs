mod sort;

fn main() {
    let mut arr = [20,120];
    sort::insert_sort(&mut arr);
    println!("{:?}", arr);
}
