use ch07_ownership::ToyVec;

fn main() {
    let mut v = ToyVec::<String>::new();
    v.push("Java Finch".to_string());
    v.push("Budgeringar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgeringar".to_string()));
}
