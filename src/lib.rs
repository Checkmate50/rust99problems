use std::fmt;

pub fn print_option(o: Option<impl fmt::Display>) {
    match o {
        None    =>  println!("None"),
        Some(s) =>  println!("{}", s)
    }
}