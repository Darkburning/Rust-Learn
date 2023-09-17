use std::collections::HashSet;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    set: HashSet<u32>, // 缓存闭包运算的结果
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            set: HashSet::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.set.contains(&arg) {
            arg
        } else {
            (self.calculation)(arg);
            self.set.insert(arg);
            arg
        }
    }
}

#[allow(unused)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 直接使用闭包，无法避免if块两次计算
    // let result = |num: u32| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // 调用函数只进行一次计算：有可能并不需要计算
    // let result = simulated_expensive_calculation(intensity);
    // 利用结构体存储闭包并将结果保存起来可以解决上述两个问题
    let mut result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", result.value(intensity));
        println!("Next, do {} situps!", result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", result.value(intensity));
        }
    }
}

pub fn closure_demo() {
    let intensity: u32 = 26;
    let random_num: u32 = 3;

    generate_workout(intensity, random_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
        assert_eq!(v1, 1);
    }

    #[test]
    fn test_closure_catch_env() {
        let x = 4;

        // can't capture dynamic environment in a fn item
        // use the `|| { ... }` closure form instead
        // fn equal_to_x(z) -> bool {z == x}
        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }
}
