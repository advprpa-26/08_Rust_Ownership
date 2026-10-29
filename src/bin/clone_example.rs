// Clone Example

// #[derive(Debug, Clone)]
// struct MyData {
//     v: Vec<i32>,
//     i: i32,
// }

#[derive(Debug)]
struct MyData {
    v: Vec<i32>,
    i: i32,
}

impl Clone for MyData {
    fn clone(&self) -> MyData {
        return MyData {
            v: self.v.clone(),
            i: self.i,
        };
    }
}

fn main() {
    let data = MyData {
        v: vec![10, 20, 30],
        i: 5,
    };
    let data_clone = data.clone();
    println!("data: {:?} clone: {:?}", data, data_clone);
}
