#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn erea(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        //Self is an alias for the type that comes after the impl keyoword : Rectangle
        // the function returns Self because there is no ; after the {};
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 22,
        height: 22,
    };

    let rect2 = Rectangle {
        width: 33,
        height: 33,
    };
    println!("the erea of the rectangle is {}", rect1.can_hold(rect2));
    let sq = Rectangle::square(3);
    println!(" squre me {:?}", sq.erea())
}
