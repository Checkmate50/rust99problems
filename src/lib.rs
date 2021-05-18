use std::fmt;

pub fn print_option<T:fmt::Display>(o: Option<T>) {
    match o {
        None    =>  println!("None"),
        Some(s) =>  println!("{}", s)
    }
}