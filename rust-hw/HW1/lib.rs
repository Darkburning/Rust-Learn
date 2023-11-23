/*
rust 作业1：
通过编写一个程序，帮助生成一些术语
将长名称（例如Portable Network Graphics）转换为其首字母缩写（PNG）。
标点符号的处理方式如下：破折号视为单词分隔符（类似于空格）；
其他所有标点符号都可以从输入中删除。
*/

fn acronym(input: &str) -> String {
    let mut res = String::new();
    let mut prev = ' ';
    for element in input.chars() {
        if prev == ' ' || prev == '-' {
            res.push(element.to_ascii_uppercase());
        }
        prev = element;
    }
    res
}
