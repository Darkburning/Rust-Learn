#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn vector_demo() {
    // vector
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    // 更新 Vec
    for i in 4..10 {
        v.push(i);
    }
    // 读取 Vec
    // 索引语法
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // 这个方法更适合当程序认为尝试访问超过 vector 结尾的元素是一个严重错误的情况，这时应该使程序崩溃
    // Panic：thread 'main' panicked at 'index out of bounds: the len is 9 but the index is 100'
    // let does_not_exist = &v[100];
    // println!("{:#?}", does_not_exist);

    // get方法
    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // 不会panic而是会返回None
    // 当偶尔出现超过 vector 范围的访问属于正常情况的时候可以考虑使用它
    // let does_not_exist = v.get(100);
    // println!("does_not_exist: {:#?}", does_not_exist);

    // 不允许获取引用后再更新，因为vec需要保证内存连续，当push的时候有可能需要
    // 分配新内存并将老的元素拷贝到新的空间中，这时，第一个元素的引用就指向了被释放的内存
    // compile error：cannot borrow `v` as mutable because it is also borrowed as immutable
    // let first = &v[0];
    // v.push(123);
    // println!("{}", first);

    // 遍历 vector 中元素的可变引用
    for i in &mut v {
        *i += 100;
    }
    // 遍历 vector 中元素的不可变引用
    for i in &v {
        println!("{}", i)
    }
    // 使用枚举来存放多种类型的值

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row.iter() {
        println!("Element: {:?}", i);
    }
}
