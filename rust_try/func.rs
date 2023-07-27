fn main() {
    let x = plus_five(3);

    println!("The value of x is {}", x);

    if x < 10 {
        println!("condition was true: x < 10");
    } else {
        println!("condition was false: x >= 10");
    }

    let condition = true;
    // 三目运算符
    // if表达式 要求if和else的值类型一致
    let number = if condition { 1 } else { 0 };
    println!("condition was true: the value of number is {}", number);

    // let mut count = 10;
    // while count != 0 {
    //     println!("countdown: {}", count);
    //     count -= 1;
    // }

    // 等价于上面的代码
    for i in (1..11).rev() {
        println!("countdown: {}", i);
    }

    println!("Go!!!");

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

    for element in months.iter() {
        println!("the month is {}", element);
    }
}

fn plus_five(x: i32) -> i32 {
    5 + x
}
