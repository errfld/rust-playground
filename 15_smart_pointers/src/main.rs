use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::List::{Cons, Nil};

fn main() {
    let list_a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("count after creating list_a: {}", Rc::strong_count(&list_a));
    let list_b = Cons(5, Rc::clone(&list_a));
    println!("count after creating list_b: {}", Rc::strong_count(&list_a));
    {
        let list_c = Cons(99, Rc::clone(&list_a));
        println!("count after creating list_c: {}", Rc::strong_count(&list_a));
        dbg!(list_c);
    }
    println!(
        "count after list_c is out of scope: {}",
        Rc::strong_count(&list_a)
    );
    dbg!(list_a);
    dbg!(list_b);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    say_hello(&m);

    let _c = DropTracePointer {
        data: String::from("my funny funny Pointer man C"),
    };
    let _d = DropTracePointer {
        data: String::from("my funny funny Pointer man D"),
    };

    println!("DropTracePointer created");
    std::mem::drop(_c);
    println!("DropTracePointer c dropped before the end of main");

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 55,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn say_hello(name: &str) {
    println!("Hello, {name}!");
}

struct DropTracePointer<T: std::fmt::Display> {
    data: T,
}

impl<T: std::fmt::Display> Drop for DropTracePointer<T> {
    fn drop(&mut self) {
        println!(
            "Dropping String from DropTracePointer with data: `{}`",
            self.data
        );
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
