#[test]
fn test_if() {
    let a = 11;
    if a % 2 == 0 {
        println!("{} is an even number", a);
    } else {
        println!("{} is an odd number", a);
    }
}

#[test]
fn test_if2() {
    let a = 12;
    let even_or_odd = if a % 2 == 0 { "an even" } else { "an odd" };
    println!("{} is {} number", a, even_or_odd);
    assert_eq!(even_or_odd, "an even");
}

fn main() {}
