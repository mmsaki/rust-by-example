fn main() {
    let a = 10;
    let b = 20;
    println!("Hello, world!, {} {}", a , b);

    // u8, u16, u32, u64, u128
    let unsined: u8 = 10;
    // i8, i16, i32, i64, i128
    let signed: i16 = -1000;
    // f32, f64
    let float: f32 = 10.0;
    println!("unsined: {}, signed: {}, float: {}", unsined, signed, float);
    
    // char is 4 bytes
    let letter = "c1232";
    let emoji = "😀";

    println!("letter: {}, emoji: {}", letter, emoji);

    let is_true = true;
    println!("is_true: {}", is_true);

    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("arr: {:?}, other_arr: {:?}", arr, other_arr.len());
    println!("other_arr: {:?}", other_arr);

    let tup: (u8, bool, f32) = (5, true, 3.2);
    let tup2 = (3, 3);

    println!("first {}, second {}, third {}", tup.0, tup.1, tup.2);
    println!("{:?}", tup2);

    let (a, b, c) = tup;

    println!("first: {}, second: {}, third: {}", a, b, c);

    // Is even function
    println!("is_even: {}", is_even(10));

    // Mut variables
    let mut mut_num = 10;
    mut_num = 20;
    println!("mut_num: {}", mut_num);

    // Borrowing and Slicing Arrays
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &arr[1..4]; // [1, 2, 3] don't know the length
    println!("slice: {:?}", slice);
    borrowing_slice(arr, slice);

    // Strings
    let str: &str = "Hello, world!";
    let mut string: String = String::from("Hello, world!");
    println!("str: {}, string: {}", str, string);

    let str_slice = &str[..6];
    let string_slice = &string[..6];
    println!("str_slice: {}, string_slice: {}", str_slice, string_slice);

    string.push('1');
    string.push_str("! 2");
    string = string.replace("Hello", "Bye");
    println!("string: {}", string);

    // If statements
    let n = 3;
    if n > 0 {
        println!("n is positive");
    } else if n < 0 {
        println!("n is negative");
    } else {
        println!("n is zero");
    }

    // For loop
    for i in 0..10 {
        println!("i: {}", i);
    }
    // While loop
    let mut i = 0;
    while i < 10 {
        println!("i: {}", i);
        i += 1;
        if i == 5 {
            println!("exit");
            break
        }
    }

    // Match
    let i = 2;
    match i {
        1 => println!("one"),
        1 | 2 => println!("one, two"),
        3..= 4 => println!("three, four"),
        5 => println!("five"),
        _ => println!("default"),
    }

    // Structs
    struct User {
        name: String,
        age: u8,
        active: bool,
    }

    impl User {
        fn new(&self, name: String, age: u8, active: bool) -> User {
            User {
                name,
                age,
                active,
            }
        }
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    let user = User {
        name: String::from("John"),
        age: 20,
        active: true,
    };

    println!("Get Name: {}",user.get_name());

    // Inheritance
    trait Citizen {
        fn is_citizen(&self) -> bool;
    }

    impl Citizen for User {
        fn is_citizen(&self) -> bool {
            true
        }
    }

    println!("is_citizen: {}", user.is_citizen());

    // Enums
    #[derive(Debug)]
    enum MyEnum {
        A,
        B(i32),
        C { x: i32, y: i32 },
    }

    let a = MyEnum::A;
    let b = MyEnum::B(10);
    let c = MyEnum::C { x: 10, y: 20 };
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    if let MyEnum::C { x, y } = c {
        println!("x: {}, y: {}", x, y);
    }

    // Vector
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("vec: {:?}", vec);

    // Map
    // use std::collections::HashMap;
    // let mut 
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}

fn borrowing_slice(arr: [u8; 10], slice: &[u8]) {
    println!("arr: {:?}", arr);
    println!("slice: {:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
