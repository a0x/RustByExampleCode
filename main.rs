/*
 * This is a tutorial of Rust language.
 */
#![allow(dead_code)]

use std::fmt::{self, Formatter, Display};
use std::mem;

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
    arrays_and_slices();
    structures();
    enumeration();
    c_like();
    linked_list();
    constants();
    variable_bindings();
    mutability();
    scope_and_shadowing();
    casting();
    literals();
    interfaces();
    alias();
    expressions();
    if_else();
    loop_();
    nested_loop();
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

fn arrays_and_slices() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("array size: {}", xs.len());
    println!("array `xs` occupies {} bytes", mem::size_of_val(&xs));
    println!("array `ys` occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
}

fn structures() {
    struct Nil;
    struct Pair(i32, f64);
    struct Point {
        x: f64,
        y: f64,
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    let point: Point = Point { x: 0.3, y: 0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point{ x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        p1: Point {x: my_x, y: my_y},
        p2: point,
    };
    let _nil = Nil;
    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pari contains {:?} and {:?}", integer, decimal);
}

enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

fn enumeration() {
    use Person::*;

    fn inspect(p: Person) {
        match p {
            Skinny    => println!("Is skinny!"),
            Fat       => println!("Is fat!"),
            Height(i) => println!("Has a height of {}.", i),
            Weight(i) => println!("Has a Weight of {}.", i),
            Info {name, height} => {
                println!("{} is {} tail!", name, height);
            },
        }
    }

    let person = Height(18);
    let danny  = Weight(10);
    let dave   = Info {name: "Dave".to_owned(), height: 72};
    let john   = Fat;
    let larry  = Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red   = 0xff0000,
    Green = 0x00ff00,
    Blue  = 0x0000ff,
}

fn c_like() {
    use Number::*;
    use Color::*;

    println!("zero is {}", Zero as i32);
    println!("one is {}", One as i32);
    println!("roses are #{:06x}", Red as i32);
    println!("violets are #{:06x}", Blue as i32);
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

fn linked_list() {
    use List::*;

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }
        }
    }
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn constants() {
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The thresholdis {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}

fn variable_bindings() {
    let an_integer = 1u32;
    let a_boolean  = true;
    let unit = ();
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The complier warns about unused variable bindings; there warnings can
    // be slienced by prefixing the variable name with an underscore.
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
}

fn scope_and_shadowing() {
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function.
    {
        // This binding only exists in this block.
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // End of the block

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

fn casting() {
    #![allow(overflowing_literals)]

    let decimal = 65.4321_f32;
    let integer: u8 = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type.
    println!("1000 as a u16 is: {}", 1000 as u16);

    // Under the hood, the first 8 bit from the least significant bit (LSB) are used,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as u8 is: {}", 1000 as u8);
    println!("  -1 as u8 is: {}", (-1i8) as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the result is the same as first casting to the corresponding
    // unsigned type then taking the two's complement.
    //
    // Unless it already fits, of course.
    println!(" 128 as i16 is: {}", 128 as i16);
    println!(" 128 as i8  is: {}", 128 as i8);

    println!("1000 as a i8 is : {}", 1000 as i8);
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn literals() {
    // Suffixed literals, their types are known at initialization.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed leteral, their types depend on how they are used.
    let i = 1;
    let f = 1.0;

    // `size_of_val` return the size of a variable in bytes.
    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));
}

fn interfaces() {
    let elem = 5u8;
    let mut vec = Vec::new();

    println!("{:?}", vec);
    vec.push(elem);
    vec.push(6);
    println!("{:?}", vec);
}

fn alias() {
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

fn expressions() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };
    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn if_else() {
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, reduce by two");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}

fn loop_() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough!");
            break;
        }
    }
}

fn nested_loop() {
    #![allow(unreachable_code)]
    let mut inner = 0u16;
    println!("Entered the outer loop");
    'outer: loop {
        'inner: loop {
            inner += 1;
            print!("{} ", inner);
            if inner > 10 {
                print!("\n");
                break 'outer;
            }
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
