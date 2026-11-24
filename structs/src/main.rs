
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    // let mut user1 = User {
    //     email: String::from ("grey@gmail.com"),
    //     username: String::from("grey123"),
    //     active: true,
    //     sign_in_count: 67
    // };

    // let name = user1.username;
    // user1.username = String::from("om123");

    // let user2 = build_user(
    //     String::from("kyle@gmail.com"),
    //     String::from("Kyle123")
    // );

    // let user3 = User {
    //     email: String::from("james@gmail.com"),
    //     username: String::from("james123"),
    //     ..user2
    // };


    // struct Color(i32,i32,i32);
    // struct Point(i32,i32,i32);

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect: {:#?}",rect);


    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rec1 = Rectangle {
        width: 20,
        height: 50
    };
    
    let rec2 = Rectangle {
        width: 40,
        height: 60
    };
    println!("Can\n {:#?} \nhold\n{:#?}\n {}",rec2,rec1,Rectangle::can_hold(&rec1, &rec2))

    let rec3 = Rectangle::square(20);
}

fn build_user(email: String,username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
