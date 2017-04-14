mod ae1;

fn main() {
    let a: [i64; 4] = [2, 3, 4, 5];
    let b = &a[1..1];
    println!("{:?}", b.is_empty());

}
