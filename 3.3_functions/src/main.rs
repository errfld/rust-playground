fn main() {
    println!("Hello, world!");

    another_function(87);
    print_labeled_measurement(33, 'C');
    print_expression_functions();
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement ist: {value}{unit_label}");
}

fn print_expression_functions() {
    let some_var = {
        let some_other_var = 99; //statement (let)
        some_other_var + 1 //expression return from block NO SEMICOLON
    }; //blocks are expressions
    let twenty = twenty();
    println!(
        "Some var is {some_var} and twenty is {twenty}, both together are {}",
        some_var + twenty
    );
    let onehundred_twenty = plus_twenty(100);
    println!("This is also {onehundred_twenty}");
}

fn twenty() -> i32 {
    20
}

fn plus_twenty(x: i32) -> i32 {
    x + twenty()
}
