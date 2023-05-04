use rand::Rng;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home ip {:?}", home);
    println!("loopback is {:?}", loopback);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + match y {
        Option::Some(val) => val,
        Option::None => 0,
    };
    println!("Sum is {}", sum);

    fn add_hat() {
        println!("Adding a hat")
    }
    fn remove_hat() {
        println!("Removing a hat")
    }

    for _ in 1..30 {
        let dice_roll = rand::thread_rng().gen_range(1..=10);
        match dice_roll {
            3 => add_hat(),
            7 => remove_hat(),
            x => println!("noting happens bacause you rolled: {}", x),
        }
    }
}
