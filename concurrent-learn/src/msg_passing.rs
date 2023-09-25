use std::fmt::format;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{clone, thread};

pub fn msg_passing_demo() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = format!("Hello.");
        println!("tx send msg: {}", msg);
        // when msg was sent, it moved
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("rx received msg: {}", received);
}

pub fn msg_passing_multi_value() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            format!("Hello"),
            format!("Hi"),
            format!("I am fine!"),
            format!("What about you?"),
        ];
        // iter() :borrow iterator
        // into_iter: ownership iterator
        for (index, msg) in v.into_iter().enumerate() {
            println!("tx send msg {}: {}", index + 1, msg);
            // tx need the ownership of the value
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // use rx as iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn msg_passing_multi_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    let cnt = Arc::new(AtomicUsize::new(1));
    let cnt_clone1 = Arc::clone(&cnt);
    let cnt_clone2 = Arc::clone(&cnt);

    thread::spawn(move || {
        let v = vec![
            format!("Hello"),
            format!("Hi"),
            format!("I am fine!"),
            format!("What about you?"),
        ];
        // iter() :borrow iterator
        // into_iter: ownership iterator
        for msg in v.into_iter() {
            println!(
                "tx send msg {}: {}",
                cnt_clone1.fetch_add(1, Ordering::SeqCst),
                msg
            );
            // tx need the ownership of the value
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v = vec![
            format!("more"),
            format!("message"),
            format!("for"),
            format!("you"),
        ];
        // iter() :borrow iterator
        // into_iter: ownership iterator
        for msg in v.into_iter() {
            println!(
                "tx send msg {}: {}",
                cnt_clone2.fetch_add(1, Ordering::SeqCst),
                msg
            );
            // tx need the ownership of the value
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // use rx as iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
