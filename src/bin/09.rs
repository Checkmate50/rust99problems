// (**) Eliminate consecutive duplicates of list elements.
use std::str::from_utf8;

fn compress<T:PartialEq>(v : &mut Vec<T>) {
    let mut i = 1;
    while i < v.len() {
        if v[i-1] == v[i] {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let s = "aaaabccaadeeee";
    let mut v = s.as_bytes().to_vec();
    compress(&mut v);
    println!("{:?}", from_utf8(&v));
}