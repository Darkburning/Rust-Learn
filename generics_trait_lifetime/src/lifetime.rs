// 并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// cannot return reference to local variable `result`
// returns a reference to data owned by the current function
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn should_panic() {
    panic!("panic!!!")
}

struct Article<'a> {
    pub part: &'a str,
}

pub fn lifetime_demo() {
    let x = "hello world";
    let y = "hello";
    println!(
        "the longest_str between `{}` and `{}` is `{}`",
        x,
        y,
        longest_str(x, y)
    );

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("long long long string is long");
    //     // result的生命周期是参数中两者中较小的一个
    //     result = longest_str(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let s: &'static str = "I have a static lifetime.";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_str() {
        let x = "hello world";
        let y = "hello";
        assert_eq!(x, longest_str(x, y));
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        should_panic();
    }
}
