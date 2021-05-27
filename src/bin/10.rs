// (*) Run-length encoding of a list. Use the result of problem P09 to
//  implement the so-called run-length encoding data compression method.
//  Consecutive duplicates of elements are encoded as lists (N E) where
//  N is the number of duplicates of the element E.

fn encode<T : PartialEq+Clone> (v : &[T]) -> Vec<(T, usize)> {
    let mut res = vec![];
    for i in r99p::pack(v) {
        res.push((i[0].clone(),i.len()))
    }
    res
}

fn main() {
    let v = "aaaabccaadeeee";
    let res = encode(&v.as_bytes());
    println!("{:?}", res);
}