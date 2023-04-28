use std::io;

fn main() {
    let tuple: (i32, i32, u8) = (3409, 34984, 3);

    let (x, y, z) = tuple;

    println!("The values of tup are ({x}, {y}, {z})");

    println!("Please choose a month between 1 and 12");
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut number_of_month = String::new();
    io::stdin()
        .read_line(&mut number_of_month)
        .expect("failed to read");
    let number_of_month: usize = number_of_month
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let res = months[number_of_month - 1];
    println!("You chose: {res}");
}
