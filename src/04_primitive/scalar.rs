fn hello() {
    println!("Hello");
}

fn f1(mut n: u32) {
    n = 1;
    println!("f1: n = {}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2: n_ptr = {:p}", n_ptr);

    *n_ptr = 2;
    println!("f2: *n_ptr = {}", *n_ptr);
}

fn main() {
    let ret = hello();
    assert_eq!(ret, ());
    assert_eq!(std::mem::size_of::<()>(), 0);

    // reference
    // -----------------------------------------------

    let mut n = 0;
    println!("main: n = {}", n);

    f1(n);
    println!("main: n = {}", n);

    f2(&mut n);
    println!("main: n = {}", n);

    // raw pointer
    // -----------------------------------------------

    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');
}
