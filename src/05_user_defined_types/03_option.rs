#[test]
fn option_test() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    }
}

#[test]
fn multi_option_test() {
    assert_eq!(_add_elems(&[3, 7, 31, 127]), Some(3 + 127));
}

fn _add_elems(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}

fn main() {}
