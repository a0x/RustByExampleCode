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