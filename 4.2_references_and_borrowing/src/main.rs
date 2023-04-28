fn main() {
    let mut s1 = String::from("Hello");
    let s1_ptr = &s1;
    let len = calculate_length(s1_ptr); //giving a reference to a value (pointer to a pointer I would image)
                                        //
    println!("The length of '{s1}' is {}", len);
    println!("But '{}' should be out of scope", *s1_ptr);
    //okay got it: s1_ptr does NOT own s1. And s1_ptr is handled like any simple scalar
    add_world(&mut s1); //if I want a mutable ref I need a mutable var, which is sensible
    println!("Now s1 is '{s1}'");

    let mut s = String::from("hello");

    let r1 = &mut s; //mutable borrows are excl. only one instance of a mutable borrow may exist at any given time
                     //you can not create inmutable borrows if there is a mutabel one around
                     //let r2 = &s;
                     //let r2 = &mut s;

    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_world(some_string: &mut String) {
    //reference on a mutable String
    some_string.push_str(", world");
}
