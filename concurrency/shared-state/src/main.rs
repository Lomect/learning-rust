use std::sync::{Mutex, Arc};
use std::thread;

/// Sync 允许多线程访问
/// 通过 Send 允许在线程间转移所有权

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let head = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(head);
    }

    for th in handles {
        th.join().unwrap();
    }
}
