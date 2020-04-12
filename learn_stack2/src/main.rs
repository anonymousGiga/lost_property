//用链表实现栈
// struct node {
//     int: val,
//     struct node *next, //next = NULL
        //struct node next
// };

#[derive(Debug)]
struct StackNode<T> {
    data: T,
    next: Option<Box<StackNode<T>>>, //None
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl <T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{top: None}
    }

    fn push(&mut self, data: T) {
        let mut node = StackNode{data: data, next: None};
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let node = self.top.take();
        match node {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.data)
            }
        }
    }
}

fn main() {
    let mut ss = Stack::new();
    ss.push(1);
    ss.push(2);
    ss.push(3);
    println!("after push, ss: {:#?}", ss);

    // ss.pop();
    // ss.pop();
    // ss.pop();
    for _ in 0..4 {
        if let Some(data) = ss.pop() {
            println!("data: {}", data);
        } else {
            println!("empty");
        }
    }

    println!("after pop, ss: {:#?}", ss);
}
