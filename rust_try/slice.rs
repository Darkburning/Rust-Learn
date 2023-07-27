fn main() {
    let mut s = String::from("Hello world!");
    // compile pass
    // let word_index = first_word_v1(&s);

    // compile error: cannot borrow `s` as mutable because it is also borrowed as immutable
    let word = first_word_v2(&s);
    syntactic_sugar(&s);

    // s.clear(); // 可变引用才可清除
    // first_word_v1 之后的word_index无意义
    println!("{}", word); // 此处仍用到不可变引用word，注释该句后可编译通过
                          // println!("{}", word_index);

    let my_string = String::from("hello world");

    // // `first_word` 接受 `String` 的切片，无论是部分还是全部
    // let word = first_word_v3(&my_string[0..6]);
    // let word = first_word_v3(&my_string[..]);
    // // `first_word` 也接受 `String` 的引用，
    // // 这等同于 `String` 的全部切片
    let word = first_word_v3(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    // let word = first_word_v3(&my_string_literal[0..6]);
    // let word = first_word_v3(&my_string_literal[..]);

    // // 因为字符串字面值**就是**字符串 slice，
    // // 这样写也可以，即不使用 slice 语法！
    let word = first_word_v3(my_string_literal);
    println!("{}", word);

    // other type slice
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[0..3]; // a_slice为&[i32]
    print!("a:");
    for element in a.iter() {
        print!(" {}", element);
    }
    println!("");
    print!("a_slice:");
    for element in a_slice.iter() {
        print!(" {}", element);
    }
}

//仅在 &String 的上下文中返回值才是一个有意义的数字
fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// 使用字符串切片&str解决上下文问题，&str为不可变引用
fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
// 将字符串切片作为参数更好
fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// String slice & syntactic_sugar
fn syntactic_sugar(s: &String) {
    let hello = &s[..5];
    let world = &s[6..];
    let sentence = &s[..];
    println!("{}, {}, {}", hello, world, sentence);
}
