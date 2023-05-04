mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Strawberry"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
fn deliver_order() {}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist();

        let mut meal = crate::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        //try to access private field
        //meal.seasonal_fruit = String::from("sodfmk")

        let order1 = crate::back_of_house::Appetizer::Soup;
        let order2 = crate::back_of_house::Appetizer::Salad;
    }
}
