use std::string::ToString;

fn stringify<T: ToString>(t: T) -> String {
    t.to_string()
}

fn stringify_i32(t: i32) -> String {
    <i32 as ToString>::to_string(t)
}

fn main() {
    stringify(1i32);
}
