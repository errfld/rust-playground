fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    /*
      when assigning pointers the pointer "moves" to the new variable (shallow copy and removal)
      thus we have to clone() which is a typical heap copy
    */
    let s2 = s.clone();

    println!("{s} - {s2}");

    //types with known sizes, which are on the stack won't be copied (as they implement the Copy trait)
    //i32 f.e. or tuples consisting only of simple scalars

    let x = 5;
    let y = x;
    println!("x= {x}, y= {y}");

    let s = String::from("hello");
    takes_ownership(s);
    //println!("{s}"); would run into an error in compile time because ownership has been transferred and s is freed now

    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    println!("{s2}"); //s1 is out of scope but s2 was given back and is still in scope
}

fn takes_ownership(some_string: String) {
    println!("Some string: {}", some_string);
} //drop should be called here because some_string is ouf scope ... this would kill the string on the sender

fn takes_and_gives_back(some_string: String) -> String {
    println!("Some string who I will return: {}", some_string);
    some_string
}
