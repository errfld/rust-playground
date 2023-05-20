fn main() {
    println!("Hello, world!");
    mutable_closure();
    fn_mut_example();
}

fn mutable_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    let mut borrows_mutably = || list.push(88);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn fn_mut_example() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value.clone());
        r.width
    });
    println!("{:#?}", list);
    println!("Sort operations {}", sort_operations.len())
}
