pub fn string_demo() {
    // new函数生成空String
    let mut s = String::new();
    s.push_str("foo");
    // from函数生成String
    let mut s1 = String::from("bar ");
    s1.push_str(&s);
    // 字符串字面量转为String
    let str = "hello world!!!";
    let s2 = str.to_string();

    println!("s is {}", s);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // 拼接字符串
    // + 运算符 注意：运算符左边为值右边为引用
    let hello = String::from("hello ");
    let world = String::from("world!");
    // 会将&String 解引用强制转换为 &str 也即：&world -> &world[..]
    // 会获得第一个参数的所有权，可以多个 + 但只会获得第一个参数的所有权
    let s3 = hello + &world;

    println!("s3 is {}", s3);
    // format！宏
    // format！不会获得所有权
    let tick = String::from("tick");
    let ta = String::from("ta");
    let tou = String::from("tou");

    let tick_ta_tou = format!("{}-{}-{}", tick, ta, tou);
    println!("tick_ta_tou is {}", tick_ta_tou);

    // Rust 不允许使用索引获取 String 字符
    // let s0 = String::from("hello");
    // let h = s1[0];
    // Rust 允许使用索引获取String Slice 但是不恰当的索引可能导致程序崩溃
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
    // let halo = "Здравствуйте";
    // let s0 = &halo[0..1];
    // println!("s0 is {}", s0);

    // 遍历
    // 返回字节byte，实际是u8
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // 返回标量值char
    // 有效的 Unicode 标量值可能会由不止一个字节组成
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
