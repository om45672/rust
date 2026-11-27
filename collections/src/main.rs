fn main() {
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v = vec![1,2,3,4,5];

    // {
    //     let mut v2 = vec![1,2,3];
    // }


    // // let third = &v[2]; **Unsafe
    // // v.push(6) // can't take mut ref when already taken immut
    // // println!("The third element is {}",third);

    // match v.get(20){
    //     Some(third) => println!("The third element is {}",third),
    //     None => println!("There is no third element"),
    // }

    // for i in &v {
    //     println!("{}",i);
    // }

    // for i in &mut v {
    //     *i += 50;
    //     println!("{}",i);
    // }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}",i),
        _ => println!("Not an integer"),
    }
}
