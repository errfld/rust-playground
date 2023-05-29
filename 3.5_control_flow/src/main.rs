fn main() {
    println!("3.5 Control Flow");
    if_expressions();
    loops();
    test_convert_temperatur();
    test_fib();
}

fn if_expressions() {
    let number = 345;

    if number < 1000 {
        //if does the right thing and expects boolean, not some "falsie/truthy"-BS
        println!("number is smaller than 1000 (true)");
    } else {
        println!("number is greater or equals 1000 (false)");
    }
    //typical else if behaviour (ommited for simplicity)

    let condition1 = true;

    let number = if condition1 { 5 } else { 6 };
    println!("if is an expression! And number is: {number}");
    // return type of expression should be the same to ensure behaviour at compile time
}

fn loops() {
    let mut temp_counter = 0;
    loop {
        //seems to behave like "while(true){}", but as an expression :-)
        println!("all day");
        temp_counter += 1;
        if temp_counter > 20 {
            break;
        }
    }

    let mut counter1 = 0;

    let res1 = loop {
        counter1 += 1;
        if counter1 == 10 {
            break counter1 * 2;
        }
    };
    println!("the looped result is: {res1}");

    let mut counter2 = 0;
    'counting_up: loop {
        println!("count = {counter2}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter2 += 1;
    }

    println!("while loops are no expression, breaking results out does not work");
    println!("COUNTDOWN with while");
    let mut counter3 = 10;
    while counter3 > 0 {
        println!("count: {counter3}");
        counter3 -= 1
    }
    println!("while LIFTOFF!");

    println!("for loop");
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("the value of element is: {element}");
    }

    println!("CONTDOWN with for-loop");
    for count in (1..10).rev() {
        println!("{count}");
    }
    println!("for-loop LIFTOFF!");
}

/*
Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
 */

fn farenheit_to_celsius(degree_in_farenheit: f64) -> f64 {
    (degree_in_farenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(degree_in_celsius: f64) -> f64 {
    degree_in_celsius * 9.0 / 5.0 + 32.0
}

fn convert_temperature((temprature, unit): (f64, char)) -> (f64, char) {
    if unit == 'C' {
        (celsius_to_farenheit(temprature), 'F')
    } else if unit == 'F' {
        (farenheit_to_celsius(temprature), 'C')
    } else {
        (0.0, 'X')
    }
}

fn test_convert_temperatur() {
    let (temp_c, unit_c) = convert_temperature((22.0, 'F'));
    println!("22F is {temp_c}{unit_c}");
    let (temp_f, unit_f) = convert_temperature((22.0, 'C'));
    println!("22C is {temp_f}{unit_f}");
}

fn fib(n_th: u32) -> u128 {
    if n_th < 1 {
        return 0;
    }
    match n_th {
        1 => 0,
        2 => 1,
        a => fib(a - 1) + fib(a - 2),
    }
}

fn fib_iterative(n_th: u32) -> u128 {
    if n_th < 2 {
        return 0;
    }
    let mut res: u128 = 1;
    let mut res_1: u128 = 0;
    for _x in 2..n_th {
        let tmp = res;
        res += res_1;
        res_1 = tmp;
    }
    res
}

fn test_fib() {
    let fib_of = 8;
    let x = fib(fib_of);
    println!("fib for {fib_of} is {x}");
    let x = fib_iterative(fib_of);
    println!("fib_iter for {fib_of} is {x}");
}
