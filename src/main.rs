fn main() {
    println!("Hello, world!");
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    println!("{:?}", v1);
    how_box_works(v1[0]);
    how_rc_arc_works(v1[2]);
    how_borrowrefference_works(&v1[3]);
    let el = v1[3];
    v1.push(5);
    println!("{}", v1[3]);
    println!("{}", el);
}

// unique pointer in c++
fn how_box_works(x: i32) {
    let b = Box::new(x);
    println!("Box value: {}", b);
}

// shared pointer in c++
fn how_rc_arc_works(x: i32) {
    let rc = std::rc::Rc::new(x);
    println!("Rc value: {}", rc);
}

// reference in c++
fn how_borrowrefference_works(x: &i32) {
    println!("Reference value: {}", x);
}
