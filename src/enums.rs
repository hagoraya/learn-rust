pub fn run() {
    //Creating enums
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    //Accessing enums
    let c: Coin = Coin::Penny;
    //Enums with data
    #[derive(Debug)]
    enum Coin_Data {
        Penny,
        Nickel,
        Dime { x: i32, y: i32 }, //nameless struct
        Quarter(String),
    }

    //impl in enums
    impl Coin_Data {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let c_d: Coin_Data = Coin_Data::Quarter("test".to_string());
    c_d.call();

    //Rust does not have null

    //Option<T> enum
    enum Option<T> {
        Some(T),
        None,
    }

    //Match
    match c {
        Coin::Penny => println!("{:?}", c),
        Coin::Dime => println!("{:?}", c),
        Coin::Nickel => println!("{:?}", c),
        Coin::Quarter => println!("{:?}", c),
    }

    //Value binding in match
    fn value_in_cents(coin: Coin_Data) -> u8 {
        match coin {
            Coin_Data::Penny => 1,
            Coin_Data::Dime { x, y } => 5,
            Coin_Data::Nickel => 10,
            Coin_Data::Quarter(some_string) => {
                println!("{}", some_string);
                12
            }
        }
    }

    let output = value_in_cents(c_d);
    println!("{}", output);

    //if let..else

   let some_i8 : u8 = 0;

   match some_i8{
       1 => println!("one"),
       2 => println!("two"),
       3 => println!("three"),
       _ => println!("unknown"),
   }


}
