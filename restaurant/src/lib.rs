// mod front_of_house {
//    pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn set_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
//     fn cook_order() {}
// }


// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast { 
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"), }
//         }
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

use rand::{Rng,CryptoRng,ErrorKind::Transient};
// use std::io::{self, Write};
use std::io::*;

mod front_of_house;

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     Ok(())
// }