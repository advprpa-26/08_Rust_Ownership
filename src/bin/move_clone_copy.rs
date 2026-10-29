// Look at the different ways assignments work
// depending on Copy and Clone traits.


fn primitive() {
    let a: i32 = 5;
    let b = a; // copy -> two separate oweners
    println!("a: {:?} b: {:?}", a, b);
}


#[derive(Debug)]
struct A {
    i: i32,
    bi: Box<i32>,
}

fn no_clone_no_copy() {
    let a = A {
        i: 5,
        bi: Box::new(5),
    };
    let b = a; // move, `a` not accessible afterwards 

    // println!("a: {:?} b: {:?}", a, b); // borrow of moved value: `a`
    println!("b: {:?}", b);
}

#[derive(Debug, Clone)]
struct B {
    i: i32,
    bi: Box<i32>,
}


fn clone_no_copy() {
    let b = B {
        i: 5,
        bi: Box::new(5),
    };
    let c = b.clone(); // make c the owner of a clone

    println!("b: {:?} c:{:?}", b, c);
}



#[derive(Debug, Clone, Copy)]
struct D {
    i: i32,
    b: bool,
}

fn clone_and_copy() {
    let d = D {
        i: 5,
        b: true,
    };
    let e = d; // no clone required

    println!("d: {:?}, e:{:?}", d, e);
}

fn main() {
    primitive();
    no_clone_no_copy();
    clone_no_copy();
    clone_and_copy();
}
