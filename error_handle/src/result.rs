#![allow(unused)]
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn result_demo() {
    // let file_name = "hello.txt";
    // Result<T, E>
    // let f = File::open(file_name);

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(file_name) {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Failed to Create the file: {}", e),
    //         },
    //         other_error => panic!("Failed to open the file: {}", other_error),
    //     },
    // };

    // unwrap 和 expect
    // 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    // let f0 = File::open(file_name).unwrap();

    // expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数
    // 可以自定义错误信息方便Debug
    // let f1 = File::open(file_name).expect("Failed to open hello.txt");

    // 将错误上抛给该函数处理
    let f2 = read_username_from_file();

    match f2 {
        Ok(s) => println!("Ok! s is {}", s),
        Err(e) => panic!("Err! e is {}", e),
    }
}

// 错误传播/错误上抛给调用者处理
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// // 错误传播简写 `?`运算符
fn read_username_from_file() -> Result<String, io::Error> {
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // `?`支持链式调用
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

    // 更为简单的函数实现
    // fs::read_to_string("hello.txt")
}
