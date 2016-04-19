/*
 * This is a tutorial of Rust language.
 */

use std::fmt::{self, Formatter, Display};

fn main() {
  println!("Hello World!");
  println!("I'm a Rustacean!");

  let x = 5 + 5;
  println!("Is `x` 10 or 100? x = {}", x);

  formatted_print();
  debug();
  display();
  testcase();
  formatting();
  literals_and_operators();
  tuples();
}

fn formatted_print() {
  println!("{} days", 31);
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  println!("{subject} {verb} {object}",
           subject = "The quick brown fox",
           verb = "jumps over",
           object = "the lazy dog.");
  println!("{} of {:b} people know binary, the other half don't.", 1, 2);
  println!("{number:>width$}", number=1, width=6);
  println!("{number:>0width$}", number=1,width=6);
  println!("My name is {0}, {1} {0}.", "Bond", "James");

  println!("Pi is roughly {0:.3}", 22.0/7.0);
  println!("Pi is roughly {1:.0$}", 3, 22.0/7.0);
  println!("Pi is roughly {:.*}", 3, 22.0/7.0);
  println!("Pi is roughly {1:.*}", 3, 22.0/7.0);
}

fn debug() {
  #[derive(Debug)]
  struct Structure(i32);

  #[derive(Debug)]
  struct Deep(Structure);

  println!("{:?} months in a year.", 12);
  println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christina", actor="actor's");
  println!("Now {:?} will print!", Structure(3));
  println!("Now {:?} will print!", Deep(Structure(7)));
}

fn display() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2 {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             big = big_range,
             small = small_range);

    let point = Point2 {x: 3.3, y: 7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {real: 3.3, imag: 7.2};
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn testcase() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Dereference `self` and create a reference to `vec`
            // via destructuring.
            let List(ref vec) = *self;
            try!(write!(f, "["));

            // Iterate over `vec` in `v` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma
                // before calling `write!`. Use `try!` to return on errors.
                if count != 0 { try!(write!(f, ", ")); }
                try!(write!(f, "{}", v));
            }

            // Close the opened bracket and return a fmt::Result value
            write!(f, "]")
        }
    }

    let v = List(vec![1,2,3]);
    println!("{}", v);
}

fn formatting() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3} -> {}, {:.3} -> {}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
                   self.red, self.green, self.blue, self.red, self.green, self.blue)
        }
    }

    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722},
        City {name: "Oslo", lat: 59.95, lon: 10.75},
        City {name: "Vancouver", lat: 49.25, lon: -123.1},
    ].iter() {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}

fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true AND false is {}", true && false);
    println!("true OR  false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR  0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("One million is written as {}", 1_000_000u32);
}

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (integer, boolean) = pair;
    (boolean, integer)
}


fn tuples() {
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )",
                   self.0, self.1, self.2, self.3)
        }
    }

    fn transpose(matrix: Matrix) -> Matrix {
        let Matrix(a, b, c, d) = matrix;
        Matrix(a, c, b ,d)
    }

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is{:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix: \n{}", matrix);
    println!("Transpose: \n{}", transpose(matrix));
}
