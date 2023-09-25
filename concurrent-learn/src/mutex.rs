use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_demo() {
    let cnt = Mutex::new(5);

    {
        let mut num = cnt.lock().unwrap();

        *num = 6;
    }
    println!("cnt = {:?}", cnt);
}

pub fn arc_demo() {
    let cnt = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let cnt = Arc::clone(&cnt);
        let handle = thread::spawn(move || {
            let mut cnt_ref = cnt.lock().unwrap();

            *cnt_ref += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("final value is {:?}", *cnt.lock().unwrap());
}
