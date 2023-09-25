use std::time::Duration;
use std::{thread, vec};
#[allow(unused)]
pub fn spawn_demo() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("spawn thread count: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("vec: {:?}", v);
    });

    for i in 1..5 {
        println!("main thread count: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
