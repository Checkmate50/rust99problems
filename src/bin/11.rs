// (*) Modified run-length encoding.

fn encode_modified<T : PartialEq+Clone> (v : &[T]) -> Vec<r99p::ListEncoding<T>> {
    let mut res = vec![];
    for i in r99p::pack(v) {
        if i.len() == 1 {
            res.push(r99p::ListEncoding::Single(i[0].clone()))
        }
        else {
            res.push(r99p::ListEncoding::Multiple(i[0].clone(), i.len()))
        }
    }
    res
}

fn main() {
    let v = "aaaabccaadeeee";
    let res = encode_modified(&v.as_bytes());
    println!("{:?}", res);
}