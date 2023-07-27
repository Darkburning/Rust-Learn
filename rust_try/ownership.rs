fn main() {
    // 在Stack和Heap中行为不同
    // Stack Example
    let x = 4;
    let y = x;

    println!("x: {}", x);
    println!("y: {}", y);

    // Heap Example
    // String example
    let mut s = String::from("hello");
    s.push_str(" world!");
    println!("{}", s);

    // Move (not just shallow copy, origin variable is invalid)
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("s1:{}", s1); // will compile error:borrow of moved value: `s1`
    println!("s2: {}", s2);

    // Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3: {}", s3);
    println!("s4: {}", s4);

    let s = String::from("ownership was taken"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let z = 233; // z 进入作用域

    makes_copy(z); // z 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 z

    let str1 = gives_ownership(); // gives_ownership 将返回值
    println!("str1: {}", str1); // 移给 str1

    let str2 = String::from("hello"); // str2 进入作用域

    let str3 = takes_and_gives_back(str2); // str2 被移动到
    println!("str3: {}", str3); // takes_and_gives_back 中,
                                // 它也将返回值移给 str3

    let str = String::from("USE IT!");
    let (return_str, len) = calculate_length(str); // 使用但不获得所有权，有点麻烦，引入引用
    println!("return_str: {}, the length of it is: {}", return_str, len);
} // 这里, str3 移出作用域并被丢弃。str2 也移出作用域，但已被移走，
  // 所以什么也不会发生。str1 移出作用域并被丢弃
  // 这里, z 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("gives_ownership"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
