mod stack;
use stack::LockFreeStack;

fn main() {
    let stack = LockFreeStack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    for _ in 0..stack.size() {
        println!("{:?}", stack.pop());
    }
    drop(stack);
}
