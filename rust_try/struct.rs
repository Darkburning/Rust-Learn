// define struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // 注意：普通struct此处应为expression，不能有分号

fn build_user(email: String, username: String) -> User {
    User {
        // syntax sugar: field init shorthand
        email,    // 等价于 email: email,
        username, // 等价于 username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // instantiate struct
    let mut user1 = User {
        email: String::from("someemail@gmail.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");
    // instantiate struct from fn
    let mut user2 = build_user(
        "someemail@example.com".to_string(),
        "someusername".to_string(),
    );

    // struct update syntax
    // without struct update syntax
    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // with struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // 注意：此expression必须放在最后
                // 在创建 user3 后不能再使用 user1的username字段，因为 user1 的 username 字段中的 String 移动（move）到 user3 中
                // 若我们给 user3 的email和username字段都赋新值则user1仍有效
                // active和sign_in_count 实现了copy trait 因此无需考虑移动
    };
    // compile error:  borrow of moved value: `user1.username`
    // println!("{}", user1.username);

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like structs
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
