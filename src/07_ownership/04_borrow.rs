#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= 1;
}

fn borrow_test() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);
    eprintln!("p1: {:?}", p1);
}

fn main() {
    borrow_test();
}
