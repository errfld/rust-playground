/*
Encapsulation
 */

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

enum UpdateCommand {
    Add(i32),
    Remove(i32),
}

impl Default for AveragedCollection {
    fn default() -> Self {
        AveragedCollection::new()
    }
}

impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average(UpdateCommand::Add(value));
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average(UpdateCommand::Remove(value));
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self, command: UpdateCommand) {
        if self.list.is_empty() {
            self.average = 0.0;
            return;
        }
        match command {
            UpdateCommand::Add(value) => {
                self.average = (self.average * (self.list.len() - 1) as f64 + value as f64)
                    / self.list.len() as f64
            }
            UpdateCommand::Remove(value) => {
                self.average = (self.average * (self.list.len() + 1) as f64 - value as f64)
                    / self.list.len() as f64
            }
        }
    }
}

//Polymorphism with trait objects

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Draw Button '{}' (w/h): ({}/{})",
            self.label, self.width, self.height
        );
    }
}
