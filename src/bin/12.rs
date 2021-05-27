// (**) Decode a run-length encoded list.

fn decode<T : PartialEq+Clone> (v : &[r99p::ListEncoding<T>]) -> Vec<T> {
    let mut res = vec![];
    for i in v {
        match i {
            r99p::ListEncoding::Single(s) => res.push(s.clone()),
            r99p::ListEncoding::Multiple(s, v) => (for _ in 0..*v {
                res.push(s.clone())
            })
        }
    }
    res
}

fn main() {
    let v = vec![   r99p::ListEncoding::Multiple('a', 4),
                                        r99p::ListEncoding::Single('b'),
                                        r99p::ListEncoding::Multiple('c', 2),
                                        r99p::ListEncoding::Multiple('a', 3),
                                        r99p::ListEncoding::Single('d'),
                                        r99p::ListEncoding::Multiple('c', 4)];
    let res = decode(&v);
    println!("{:?}", res);
}