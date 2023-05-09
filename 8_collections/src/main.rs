use std::collections::btree_map::Values;

fn main() {
    //create an empty vector
    // let v: Vec<i32> = Vec::new();
    //we dont use it, but now I shadow it
    let v = vec![6, 3, 9];

    let third = &v[2]; //type inference making third &i32
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is still {third}"),
        None => println!("Now there is no thrid element anymor"),
    }

    //mutable
    let mut v = Vec::new();
    //without pushing a specific type rust can not inference type
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //let does_not_exist = &v[100]; //panics with index out of bounds
    let does_not_exist = &v.get(100);
    match does_not_exist {
        Some(_) => println!("This should not happen"),
        None => println!("There is no element at index 100"),
    }
}
