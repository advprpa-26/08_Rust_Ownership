// DIY: Explore the Rules
// Try to answer the questions by analyzing/constructing examples.
fn main() {
    owner_is_mutable();
    owner_is_immutable();
}

/*
1. Can immutable references be in scope?
2. More than one?
3. Can the value be mutated via owner when immutable references are in scope?
4. Can mutable references be in scope?
5. More than one?
6. Can the value be mutated via owner when a mutable reference is in scope?
7. Can mutable and immutable refereces be in scope at the same time?
*/
fn owner_is_mutable() {
    let mut s = String::from("Hi");
    let is1 = &s;
    //let is2 = &s;
    //let ms = &mut s;
    //s.push_str(", world");
    
    println!("{s}");
    println!("{is1}");
    //println!("{is2}");
    //println!("{ms}");
}

/*
8. Can immutable references be in scope? 
9. More than one?
10.Can mutable references be in scope?
11.More than one?
*/
fn owner_is_immutable() {
    let s = String::from("Hi");
    let is1 = &s;
    //let ms = &mut s;
    println!("{s} {is1}")
}