fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> Result<&T, &str> {
    let mut largest = match list.get(0) {
        Some(val) => val,
        None => return Err("Empty list"),
    };
    for item in list {
        if item > largest {
            largest = item
        }
    }
    Ok(largest)
}

pub trait Wiener<T> {
    fn largest(&self) -> &T;
}

//Blanket implementation
impl<T: std::cmp::PartialOrd> Wiener<T> for Vec<T> {
    fn largest(&self) -> &T {
        let mut largest = &self[0];
        for item in self {
            if item > largest {
                largest = item
            }
        }
        largest
    }
}

fn main() {
    let number_list = vec![54, 12, 9, 102, 33, 99, 100];
    let result = largest_i32(&number_list);
    let gen_result = largest(&number_list).unwrap_or_else(|err_message| panic!("{}", err_message));
    println!(
        "the largest number is: {} and with generic: {}",
        result, gen_result
    );
    let empty_list: Vec<i32> = vec![];
    match largest(&empty_list) {
        Ok(empty_result) => println!("the largest number of an empty list: {}", empty_result),
        Err(msg) => println!("Error: {}", msg),
    }

    let char_list = vec!['y', 'm', 'c', 'a'];
    let result = largest_char(&char_list);
    let gen_result = largest(&char_list).unwrap_or_else(|err_message| panic!("{}", err_message));
    println!(
        "the largest char is: {} and with generic: {} and with trait: {}",
        result,
        gen_result,
        char_list.largest()
    )
}
