/// 并发编程（Concurrent programming），代表程序的不同部分相互独立的执行
/// 并行编程（parallel programming）代表程序不同部分于同时执行

use std::thread;
use std::time::Duration;

fn main() {
    let head = thread::spawn(||{
        for i in 0..10 {
            println!("spawn thread is: {}", i);
            thread::sleep(Duration::from_millis(30));
        }
    });

    for i in 0..5 {
        println!("main thread is: {}", i);
        thread::sleep(Duration::from_millis(30));
    }

    head.join().unwrap();

    let a = String::from("lome");
    let handle = thread::spawn(move || {
        println!("a is: {}",a);
    });

    handle.join().unwrap();
//    println!("a {}",a);
}
