trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

trait As<T> {
    fn cast(self) -> T;
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

fn main() {
    //  generics trait
    // -----------------------------------------------
    let data = Box::init("foo");
    let data = Box::<f32>::init(0.1);
    let data: Box<f32> = Init::init(0.1);

    let one_u32: u32 = 1.cast();
    let one_u32: u64 = 1.cast();
}
