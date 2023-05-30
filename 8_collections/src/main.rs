use std::collections::HashMap;

use rand::Rng;

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
    let mut v = vec![4, 5, 6, 7, 8];

    //let does_not_exist = &v[100]; //panics with index out of bounds
    let does_not_exist = &v.get(100);
    match does_not_exist {
        Some(_) => println!("This should not happen"),
        None => println!("There is no element at index 100"),
    }

    let first = &v[0];

    //v.push(100); //cannot borrow as mutable because it is also borrowed as immutable Book 8.1

    println!("The first element is: {}", first);

    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }

    let mut s = String::new();
    let data = "some string";
    let string_data = data.to_string();

    s.push_str(data);

    println!("s=\"{s}\" and string_data=\"{string_data}\"");

    let combined_string = s + " " + &string_data;

    println!("combined_string:{combined_string}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let sg = s1 + "-" + &s2 + "-" + &s3;
    println!("Gesamt: {sg}"); //s1 can not be borrwed because it ist gone taken by sg and the Add method

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let sg = format!("{s1}-{s2}-{s3}");
    println!("Gesamt: {sg} Einzeln: {s1}, {s2}, {s3}");

    for c in sg.chars() {
        println!("{c}");
    }

    let mut facepalm_per_day = HashMap::new();

    facepalm_per_day.insert("monday".to_string(), 9);
    facepalm_per_day.insert("tuesday".to_string(), 12);
    facepalm_per_day.insert("wednesday".to_string(), 4);
    facepalm_per_day.insert("thursday".to_string(), 7);
    facepalm_per_day.insert("friday".to_string(), 15);

    for (key, value) in &facepalm_per_day {
        println!("{key}: {value}");
    }
    let saturday = facepalm_per_day.get("saturday").unwrap_or(&0);
    println!("number of facepalms on saturday: {saturday}");
    println!("{:?}", facepalm_per_day);
    facepalm_per_day.entry("saturday".to_string()).or_insert(6);
    println!("{:?}", facepalm_per_day);

    let days = vec![
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    for day in days {
        let count = facepalm_per_day.entry(day.to_string()).or_insert(0);
        *count += rand::thread_rng().gen_range(1..=10)
    }
    for (key, value) in &facepalm_per_day {
        println!("{key}: {value}");
    }
}
