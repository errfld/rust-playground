fn main() {
    let x = longer("ab", "aac");
    println!("longer: {x}");
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn not_in_scope_example() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r);
// }
