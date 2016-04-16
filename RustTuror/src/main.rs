/*
 * This is a tutorial of Rust language.
 */

use std::fmt;

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
    use std::fmt::{self, Formatter, Display};

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