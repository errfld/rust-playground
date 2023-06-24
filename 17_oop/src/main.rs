use oop::{AveragedCollection, Button, Draw, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    collection.add(4);
    let average1 = collection.average();
    collection.remove();
    let average2 = collection.average();
    println!("average1: {}, average2: {}", average1, average2);
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Draw SelectBox (w/h): ({}/{}) with options: {:?}",
            self.width, self.height, self.options
        )
    }
}
