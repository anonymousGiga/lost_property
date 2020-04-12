#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
    capacity: usize,
}

impl <T> Queue<T> {
    //fn new() -> Self {
    fn new(size: usize) -> Self {
        Queue{ 
            qdata: Vec::with_capacity(size),
            capacity: size,
        }
    }

    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.qdata.len() == self.capacity {
            //Err("No space in queue!".to_string())
            return Err("No space in queue!".to_string());
        }

        self.qdata.push(item);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        let size = self.qdata.len();
        if size > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.qdata.len()
    }
}

fn main() {
    let mut q = Queue::new(2);
    q.enqueue(1);
    println!("1 enqueue!");
    q.enqueue(2);
    println!("2 enqueue!");

    if let Err(error) = q.enqueue(3) {
        println!("enqueue error: {}", error);
    }

    println!("++++++++++++++++++");

    println!("size: {}", q.size());
    println!("q: {:#?}", q);

    println!("++++++++++++++++++");

    for _ in 0..3 {
        if let Some(data) = q.dequeue() {
            println!("data: {}", data);
        } else {
            println!("queue empty!");
        }
    }

}
