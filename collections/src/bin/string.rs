use unicode_segmentation::UnicodeSegmentation;

fn main(){
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');
    // println!("String: {}",s);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World");
    // // let s3: String = s1+&s2;
    // let s3 = format!("{}{}",s1,s2);

    // let hello = String::from("Здравствуйте");
    // let c: char = hello[0];    

    let hello: String = String::from("नमस्ते");

    // for b in "नमस्ते".bytes() { // scalar values
    //     println!("{}",b);

    for b in "नमस्ते".chars() { // char values
        println!("{}",b);
    }

    for b in "नमस्ते".graphemes(true) {
        println!("{}",b);
    }
}