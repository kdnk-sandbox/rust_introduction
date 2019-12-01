#[test]
fn test_match() {
    let unknown = Some("Apple");
    let string = match unknown {
        Some(something) => String::from("Hi, ") + something,
        None => String::from("Nothing"),
    };
}

fn main() {}
