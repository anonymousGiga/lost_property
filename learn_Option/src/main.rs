use std::thread;
use std::time::Duration;

struct Worker {
    // thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new() -> Worker {
        let thread = thread::spawn(move || {
            println!("start work 10 secs ... ");
            thread::sleep(Duration::from_secs(10));
            println!("work 10 secs finish... ");
        });

        Worker { thread: Some(thread) }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new());
        }

        ThreadPool{workers}
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // worker.thread.join().unwrap();
            // println!("worker thread finished!");
            if let Some(thread) = worker.thread.take() {//worker.thread: (Some(thread)) -> None  
                thread.join().unwrap();
                println!("worker thread finished!");
            }
        }
    }
}

fn main() {
    // let _pool = ThreadPool::new(3);

    let n: Option<i32> = Some(2);
    if !(n.is_some()) {
        println!("nothing!");
    }

    let text: Option<String> = Some("Hello, world!".to_string());

    let text_length: Option<usize> = text.as_ref().map(|s| s.len());

    println!("still can print text: {:?}", text);
    if let Some(length) = text_length {
        println!("string leng = {}", length);
    }

    let mut x = Some(2);
    let y = x.take(); //x:None, y:Some(2)
    if x.is_some() {//x nothing
        println!("x some!");
    } else {
        println!("x nothing!");
    }

    if y.is_none() { //y some
        println!("y nothing!");
    } else {
        println!("y some!");
    }


    println!("Hello, world!");
}
