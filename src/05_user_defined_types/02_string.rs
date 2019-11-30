#[test]
fn string_test() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");
    let mut s3 = "ストロベリー".to_owned();

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");
    println!("{}", s1);

    s2.push('と');

    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");
}

fn main() {}
