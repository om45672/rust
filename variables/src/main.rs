fn main() {
    // let  x = 5;
    // println!("The value of x is: {}",x);
    // let x = "six";
    // println!("The value of x is: {}",x);

    // const SUBSCRIBER_COUNT: u32 = 100_000;

    // let tup = ("B&W Donut",100_000);
    // let (channel,sub_count) = tup;
    // let sub_count = tup.1;

    // let sum = my_function(11,22);
    // println!("The sum of x and y is {}", sum);

    // // control flow
    // let number = 5;

    // if number < 10 {
    //     println!("First contition was true");
    // } else if number < 22 {
    //     println!("Second condition is true");
    // } else {
    //     println!("condition was false");
    // }

    // let condition = true;
    // let number = if condition {5} else {6};

    // let mut counter = 0;
    // let res = loop {
    //     counter+=1;
    //     if counter == 10 {break counter;}
    // };
    // println!("The result is {}",res);

    // let mut number = 3;
    // while number!=0 {
    //     println!("{}!", number);
    //     number-=1;
    // }
    // println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
    let x = 1;
    let y=4;
    for number in x..y {
        println!("{}!",number);
    }
}

// fn my_function(x: i32,y: i32) -> i32{
//     println!("The value of x is {}", x);
//     println!("The value of y is {}", y);
//     x+y
// }