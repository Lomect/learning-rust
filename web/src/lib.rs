use std::thread;
use std::sync::{mpsc, Mutex, Arc};


enum Message {
    NewJob(Job),
    Down,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}
type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rv) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let rv= Arc::new(Mutex::new(rv));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rv)));
        }

        ThreadPool {
            workers,
            sender: tx
        }
    }

    // Fn 借用环境变量  FnOnce 移动环境变量所有权到闭包中 FnMut 可以改变环境变量闭包获取到的是&mut
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {

        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Down message to all Worker");

        for _ in &mut self.workers {
            self.sender.send(Message::Down).unwrap();
        }

        println!("Shutting Down all workers");
        for worker in &mut self.workers {
            println!("Shut Down worker {}", worker.id);
            if let Some(th) = worker.thread.take() {
                th.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rv: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = rv.lock().unwrap().recv().unwrap();
                println!("Worker {} get a job; execting", id);

                match job {
                    Message::NewJob(job) => {
                        job.call_box();
                    },
                    Message::Down => {
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}