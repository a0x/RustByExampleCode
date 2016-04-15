/*
 * This is a tutorial of Rust language.
 */


fn main() {
  println!("Hello World!");
  println!("I'm a Rustacean!");
  
  let x = 5 + 5;
  println!("Is `x` 10 or 100? x = {}", x);
  
  formatted_print();
  debug();
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