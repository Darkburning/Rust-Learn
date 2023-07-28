#[derive(Debug)] // 在结构体定义之前加上外部属性Debug
struct Rectangle {
    width: u32,
    height: u32,
}
// 使用impl块实现结构体方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数不以self为第一参数，常用作构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
// 普通函数
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Derive Debug trait
    println!("Debug trait1: rect1 is {:?}", rect1);
    println!("Debug trait2: rect1 is {:#?}", rect1);

    println!(
        "Normal Method: The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "Struct Method: The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!(
        "Struct Method: The area of the square is {} square pixels.",
        square.area()
    );
}
