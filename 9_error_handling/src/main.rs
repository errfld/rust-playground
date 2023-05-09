use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    //panic!("let the world burn!") //causes instantanous application death
    println!("{}", read_funny_bunny_file_short().unwrap());
    read_funny_bunny_file();
    try_to_create_the_file_closure();
    try_to_create_the_file();
    panic_without_file();
}

fn try_to_create_the_file() {
    let funny_bunny_file_result = File::open("bunny.txt");
    let funny_bunny_file = match funny_bunny_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("bunny.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem while creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("{:?}", funny_bunny_file);
}

fn try_to_create_the_file_closure() {
    let funny_bunny_file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem while creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err)
        }
    });
    println!("{:?}", funny_bunny_file);
}

fn panic_without_file() {
    let funny_bunny_file_result = File::open("bunny.txt");
    let funny_bunny_file = match funny_bunny_file_result {
        Ok(file) => file,
        Err(error) => panic!("this file kills applications because: {:?}", error),
    };
    println!("{:?}", funny_bunny_file);
}

fn read_funny_bunny_file_short() -> Result<String, Error> {
    let mut content = String::new();
    File::open("funny_bunny.txt")?.read_to_string(&mut content)?;
    Ok(content)
}

fn read_funny_bunny_file() {
    match read_funny_file("funny_bunny.txt") {
        Ok(text) => print!("{}", text),
        Err(err) => panic!("We did bad ... {:?}", err),
    };
}

fn read_funny_file(filename: &str) -> Result<String, Error> {
    let mut funny_file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut content = String::new();
    match funny_file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}
