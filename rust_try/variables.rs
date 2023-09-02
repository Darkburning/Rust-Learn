// 常量全使用大写字母，可在任何作用域内声明，类型必须标注
// 常量只可以绑定到常量表达式
const MAX_NUMBER: u32 = 100_000;
fn main() {
    let x = 5; // 默认不可变

    println!("The previous value of x is {} ", x);

    let x = 6; // shadow

    println!("The current value of x is {} ", x);

    let mut x = 5; // mut 使之可变

    println!("The previous value of x is {} ", x);

    x = 6;

    println!("The current value of x is {} ", x);

    println!("The value of MAX_NUMBER is {} ", MAX_NUMBER);

    let spaces = "    ";
    println!("previous spaces is {}", spaces);
    // shadow 不同于将变量标记为mut 类型可与之前不同 仍不可变
    let spaces = spaces.len();

    println!("current spaces is {}", spaces);

    // TUPLE 长度固定
    let tuple: (i32, f64, u8, bool) = (500, 6.4, 1, false);
    println!("VISIT: {}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);

    // destruct
    let (u, v, w, x) = tuple;

    println!("DESTRUCT: {}, {}, {}, {}", u, v, w, x);

    // ARRAY 长度固定
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("First Month is {}", months[0]);

    for element in months.iter() {
        println!("the month is {}", element);
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the element is {}", element);
    }

    let b = [3; 5]; // [value; times]
    for element in b.iter() {
        println!("the element is {}", element);
    }
}
