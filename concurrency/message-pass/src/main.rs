use std::thread;
use std::sync::mpsc; /// mpsc 是 多个生产者，单个消费者mpsc 是 多个生产者，单个消费者.

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("lome");
        tx.send(s).unwrap();
    });

    let rv = rx.recv().unwrap();
    println!("rv: {}", rv);

    let (txs, rxv) = mpsc::channel();
    let txs1 = mpsc::Sender::clone(&txs);
    thread::spawn(move || {
        let ve = vec![
            String::from("this"),
            String::from("is"),
            String::from("lome"),
        ];
        for i in ve {
            txs1.send(i).unwrap();
        }
    });

    let tx2 = mpsc::Sender::clone(&txs);
    thread::spawn(move || {
        let ve = vec![
            String::from("that"),
            String::from("are"),
            String::from("lory"),
        ];
        for i in ve {
            tx2.send(i).unwrap();
        }
    });

    for r in rxv {
        println!("{}", r);
    }
}
