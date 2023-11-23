fn main() {
    let s = format!("你好");

    for i in 0..s.chars().count() {
        println!("{:?}", s.chars().nth(i).unwrap());
    }

    s.chars().for_each(|c| println!("{}", c))
}
