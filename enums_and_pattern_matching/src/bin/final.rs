
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!",state);
            25
        }
    }
}

fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let some_number  = Some(5);
    // let some_string = Some("a string");

    // let absent_number: Option<i32> = None;


    // let x: i8 = 5;
    // let y: Option<i8> = None;

    // let sum = x+y.unwrap_or(0);

    // println!("Returned value: {}",value_in_cents(Coin::Quarter(UsState::California)));


    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("Three")
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        _ => None,
    }
}