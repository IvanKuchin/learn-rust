use std::{thread, time::Duration};

fn main() {

    let th = thread::spawn(|| {
        for i in 0..10 {
            println!("spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    th.join().unwrap();
}
