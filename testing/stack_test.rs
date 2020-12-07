extern crate tas;
use tas::*;

trait Stack {
    type Item;

    fn empty(&self) -> bool;
    fn depth(&self) -> usize;
    fn push(&mut self, item: Self::Item);
    fn top(&self) -> Option<&Self::Item>;
    fn pop(&mut self) -> Option<Self::Item>;
}


struct MyStack<T> {
    vec: Vec<T>,
}

impl<T> Stack for MyStack<T> {
    type Item = T;

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn depth(&self) -> usize {
        self.vec.len()
    }

    fn push(&mut self, item: Self::Item) {
        self.vec.push(item);
    }

    fn top(&self) -> Option<&Self::Item> {
        self.vec.last()
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}


impl<T> MyStack<T> {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }
}


tests!({
    test_case!(
        "A new stack is empty",
        {
            let new_stack: MyStack<i32> = MyStack::new();

            require!(new_stack.empty());
        }
    );


    test_case!(
        "An empty stack returns None when queried for its top",
        {
            let empty_stack: MyStack<i32> = MyStack::new();

            require_that!(&empty_stack.top(), equal_to(None));
        }
    );


    test_case!(
        "An empty stack returns None when popped",
        {
            let mut empty_stack: MyStack<i32> = MyStack::new();

            require_that!(&empty_stack.pop(), equal_to(None));
        }
    );


    test_case!(
        "A empty stack gains depth by pushing on it",
        {
            let mut stack: MyStack<String> = MyStack::new();

            stack.push(String::from("Item"));

            require_that!(&stack.depth(), equal_to(1));
            require_that!(stack.top().unwrap(), equal_to(String::from("Item")));
        }
    );


    test_case!(
        "A non-empty stack gets deeper by pushing on it",
        {
            let mut stack: MyStack<String> = MyStack::new();
            stack.push(String::from("Bottom Item"));

            stack.push(String::from("Top Item"));

            require_that!(&stack.depth(), equal_to(2));
        }
    );


    test_case!(
        "Popping from a non-empty stack regains tops in reverse order",
        {
            let mut stack: MyStack<String> = MyStack::new();
            stack.push(String::from("Bottom Item"));
            stack.push(String::from("Top Item"));

            stack.pop();

            require_that!(stack.top().unwrap(), equal_to(String::from("Bottom Item")));
        }
    );
});