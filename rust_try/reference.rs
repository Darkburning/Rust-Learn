fn main() {
    let s1 = String::from("hello");
    let len = calculate_length_v1(&s1);
    println!("calculate_length_v1: length of {} is: {}", s1, len);

    let mut s2 = String::from("HELLO");
    let len = calculate_length_v2(&mut s2);
    println!("calculate_length_v2: length of {} is: {}", s2, len);

    // 在同一作用域，不能拥有对同一个数据的多个可变引用
    // compile error: cannot borrow `s3` as mutable more than once at a time
    // let mut s3 = String::from("ALOHA");
    // let s4 = &mut s3;
    // let s5 = &mut s3;
    // println!("{}, {}", s4, s5);

    // 在同一作用域，可以拥有对同一个数据的多个不可变引用
    // let mut s3 = String::from("ALOHA");
    // let s4 = & s3;
    // let s5 = & s3;

    // 在同一作用域，不可以同时拥有对同一个数据的不可变引用和可变引用
    // compile error: cannot borrow `s3` as mutable, as it is not declared as mutable
    // let mut s3 = String::from("ALOHA");
    // let s4 = &mut s3;
    // let s5 = &s3;

    // compile error: missing lifetime specifier
    // let reference_to_nothing = dangle();
}

// default: reference is immutable
fn calculate_length_v1(s: &String) -> usize {
    s.len()
}
// mutable reference
fn calculate_length_v2(s: &mut String) -> usize {
    println!("mutable reference work!");
    s.push_str("ALOHA!");
    s.len()
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
