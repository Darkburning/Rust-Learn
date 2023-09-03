#![allow(unused)]
mod panic;
mod result;
use std::fs::File;

fn main() {
    // 在powershell若需要栈展开需要设置环境变量：`$env:RUST_BACKTRACE=1`
    // panic::panic_demo();
    result::result_demo();

    // ? 运算符可被用于返回 Result 的函数
    // compile:
    // the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    // let f = File::open("hello.txt")?;
}
