// (*) Split a list into two parts; the length of the first part is given.

fn split<T> (v : &[T], n : usize) -> Vec<&[T]> {
    vec![&v[..n], &v[n..]]
}

fn main() {
    let s = "abcdefghik";
    let res = split(&s.as_bytes(), 3);
    println!("{:?}", res);
}