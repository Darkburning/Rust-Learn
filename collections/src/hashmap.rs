use std::collections::HashMap;

pub fn hashmap_demo() {
    // 新建hashmap 1
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 新建hashmap 2
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 访问hashmap中的value
    // 通过get访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // 更新hashmap
    // 覆盖一个值
    println!("覆盖一个值:");
    scores.insert(String::from("Blue"), 25);
    // for循环遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 在key没有对应value时插入
    println!("在key没有对应value时插入:");
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
    // 根据旧值更新值
    let text = "hello world wonderful world hello everyone do you like the world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
