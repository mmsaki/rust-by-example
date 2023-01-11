fn main () {
    // {} is replaced by any arguments
    println!("{} days", 31);

    // Positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!("{subject} {verb} {object}", 
             object="the lazy dog", 
             subject="the quick brown fox", 
             verb="jumps over");

    // Different formating can be invoked with `:` colon
    println!("Base 10:             {}",   69420); // 69429
    println!("Base 2 (binary):     {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):      {:o}", 69420);// 207454
    println!("Base 16 (hexadecimal){:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal){:X}", 69420); // 10F2C
    
    // Justify text with specific width
    // output "     1". four whitespaces and a "1", total width of 6
    println!("{number:>6}", number=1);

    // Pad numbers with extra zeroes
    // and left-adjust by flipping the sign. This will output 1000000
    println!("{number:0<6}", number=1);

    // You can name arguments in the format specifier by appending `$`
    println!("{number:0>width$}", number=1, width=6);

    // rust checks to make sure the correct number of args are used
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that display fmt::Display can be formatted using {}
    // #[allow(dead_code)]
    // #[derive(Debug)] // this allows us to use `{:?}` format
    use std::fmt;
    struct Structure(i32);
    // rust will not compile if you dont implement fmt::Display
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct {} won't print", Structure(3));

    // using varible for formating
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
}
