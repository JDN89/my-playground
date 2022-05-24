// Methods defined withing a struct
// first Parameter always self
// dus vergelijkbaar met de methods in c#
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &self is always the first parameter
    fn can_contain(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.width > other_rect.width
    }
    // associated functions because they’re associated with the type named after the impl
    //We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with
    // To call this associated function, we use the :: syntax with the struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 20,
    };
    println!("the area of the rect1 is {}", rect1.area());
    println!("rect1 can contain rect2? {}", rect1.can_contain(&rect2));
    println!("rect 1 can containt rect3? {}", rect1.can_contain(&rect3));
    let sq = Rectangle::square(3);
    println!("the area of sq is {}", sq.area());
}
