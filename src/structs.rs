#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //&self allows us to use <rect.area>
        &self.height * &self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        &self.height > &rect.height && &self.width > &rect.width
    }
    //Does not reference self, so you will call it as Rectange::square(size)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    // * Structs
    //Creating struct

    //Instantiating a struct
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    //Access struct elements
    println!("Accessing element height: {}", rect.height);

    //change struct elements
    rect.height = 50;
    println!("Accessing element height after change: {}", rect.height);

    //printing structs
    //You must derive debug trait
    println!("{:?}", rect);

    // * Tuple structs - (nameless struct) creating a structing without naming each of its variables
    struct color(u32, u32, u32);
    let black = color(0, 0, 0);

    //Access tuple struct vars
    println!("{}", black.1);

    //Coping values from other structs
    let rect1 = Rectangle {
        width: 12,
        height: rect.height,
    };
    //or if you have alot of values that you want to copy
    let rect2 = Rectangle { width: 0, ..rect };
    println!("{:?}", rect2);

    //Creating a struct by passing its values to a function
    fn new_rect(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    let rect3 = new_rect(12, 23);
    println!("{:?}", rect3);

    // * Methods - functions that are defines within the context of a struct

    println!("{}", rect1.area());

    let rect_small = Rectangle {
        width: 2,
        height: 2,
    };
    let rect_big = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "Can rect_big hold rect_small: {}",
        rect_small.can_hold(&rect_big)
    );

    //Methods that dont reference themselfs are called like this
    let square = Rectangle::square(10);
    println!("square of 10 {:?}", square);
}
