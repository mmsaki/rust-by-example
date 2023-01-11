use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x;
        let y = self.y;
        write!(f, "{} {}", x, y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big} and the small range is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    // Both Debug and Display were implemented, but {:b} requires `fmt::Binary`
    // to be implented. The line below will not work.
    // println!("What does Point2d look like in binary: {:b}?", point);
    // Tutorial: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    /*
     * Here's how you can implement binary
     * 
     * 
     */
    struct Length(i32);

    impl fmt::Binary for Length {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let val = self.0;
            fmt::Binary::fmt(&val, f)
        }
    }

    let l = Length(107);
    assert_eq!(
        format!("l as binary is: {l:b}"), "l as binary is: 1101011");
    assert_eq!(
        format!("l as binary is: {l:#032b}"),
        "l as binary is: 0b000000000000000000000001101011"
              );

}
