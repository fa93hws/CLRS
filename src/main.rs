mod sort;

fn main() {
    let mut arr = ['a', 's', 'b'];
    sort::insert_sort(&mut arr);
    println!("{:?}", arr);
}
