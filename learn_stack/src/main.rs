//栈：后进先出
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
    top: usize,
    capacity: usize,
}

impl <T> Stack<T> {
    //fn new() -> Self {
    fn new(size: usize) -> Self {
        Stack {
            data: Vec::new(),
            top: 0,
            capacity: size,
        }
    }

    // fn push(&mut self, item: T) {
    fn push(&mut self, item: T) -> Result<(), String>{
        //if self.top >= self.data.capacity()
        if self.top >= self.capacity {
            return Err(String::from("There is no space in stack!"));
        }

        self.data.push(item);
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            //None
            return None;
        }

        self.top -= 1;
        self.data.pop()
    }

    fn top(&self) -> usize {
        self.top
    }

}
fn main() {
    let mut s = Stack::new(3);
    let _ = s.push(1);
    let _ = s.push(2);
    let _ = s.push(3);

    if let Err(error) = s.push(4) {
        println!("push 4 error: {}", error);
    }

    println!("++++++++++++++++++++");
    println!("top: {}", s.top());
    println!("s: {:#?}", s);

    for _ in 0..4 {
        let a = s.pop();
        match a {
            Some(item) => println!("pop item: {}", item),
            None => println!("stack empty!"),
        }
    }
}
