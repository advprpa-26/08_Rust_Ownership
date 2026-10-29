pub fn main() {
    let a = Box::new([0; 1000]);
    let b = a;
    println!("{:?}", b.len());
}