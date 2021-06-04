// (**) Extract a slice from a list.

fn slice<T> (v : &[T], i : usize, j : usize) -> &[T] {
    &v[i-1..j]
}

fn main() {
    let s = "abcdefghik";
    let res = slice(&s.as_bytes(), 3, 7);
    println!("{:?}", res);
}