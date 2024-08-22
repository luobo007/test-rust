use core::fmt;
use std::fmt::write;

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{:?}", rect);
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle(width={:?}, height={:?})",
            self.width, self.height
        )
    }
}
