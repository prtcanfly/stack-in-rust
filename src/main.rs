use std::vec::Vec;

// contains a single field, Vector 'Vec<T>' that is used to store elements
#[derive(Debug)]
struct Stack<T> {
    vec: Vec<T>,
}

// implement some methods to add/remove elements from stack
impl<T> Stack<T> {
    // creates a new, empty Stack
    fn new() -> Self {
        Stack { vec: Vec::new() }
    }

    // push method will add to the top of the stack
    fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }

    // pop method will remove and return the top element from the stack
    // if stack is empty, it will return none
    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    // this will return true if the stack is empty, and false if not
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

fn main() {
    // create a new empty stack that will hold i32 elements
    let mut stack: Stack<i32> = Stack::new();

    // add some elements to the stack using push method
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // check if the stack is empty
    println!("Is the stack empty? {}", stack.is_empty());
    println!("Here is the stack: {:?}", stack);

    // remove elements using the pop method
    // removes '3' and returns 'Some(3)'
    println!("Popped: {:?}", stack.pop());

    // continue removing elements
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());

    // it should now return none
    println!("Popped: {:?}", stack.pop());

    // double check using is_empty
    println!("Is the stack empty now? {}", stack.is_empty());
}
