use rand::Rng; // trait 类似其他语言的interface
use std::cmp::Ordering; // enum
use std::io; // not prelude

fn main() {
    println!("Guess the  number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut count = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // String 是prelude

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        count += 1;

        // 变量遮蔽（shadow）,用于类型转换
        // 忽略非数字的猜测
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guess {} times", count);
                break;
            }
        }
    }
}
