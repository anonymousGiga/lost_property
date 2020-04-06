use std::mem;

struct A {
	a: u8,    // 1 byte
	//padding // 3 byte
	b: u32,   // 4 byte
	c: u16,   // 2 byte
	//padding // 2 byte
}

fn main() {
	let aa = A {a: 1, b: 2, c: 3};
	println!("size = {}", mem::size_of::<A>());
	println!("size = {}", mem::size_of_val(&aa));
    println!("Hello, world!");
}
