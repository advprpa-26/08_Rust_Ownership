struct MyData {
    values: Vec<i32>,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("MyData is being deallocated!");
    }
}

fn main() {
    allocate_custom_data();
    println!("Done!");
}

fn allocate_custom_data() {
    let data = MyData { values: vec![10, 20, 30] };
    println!("MyData allocated with values: {:?}", data.values);
    // `data` will be deallocated at the end of this function, triggering the `drop` method
}
