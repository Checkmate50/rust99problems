// (**) Run-length encoding of a list (direct solution).

fn encode_direct<T : PartialEq+Clone> (v : &[T]) -> Vec<r99p::ListEncoding<T>> {
    let mut res = vec![];
    if v.len() == 0 {
        return res;
    }
    let mut prev = (&v[0], 0);
    for i in v {
        if res.len() == 0 {
            res.push(r99p::ListEncoding::Single((*i).clone()));
            prev = (i, 1);
        }
        if *prev.0 == *i {
            if prev.1 == 1 {
                res.pop();
                res.push(r99p::ListEncoding::Multiple((*i).clone(), 2));
                prev.1 = 2;
            }
            else {
                res.pop();
                res.push(r99p::ListEncoding::Multiple((*i).clone(), prev.1+1));
                prev.1 += 1;
            }
        }
        else {
            res.push(r99p::ListEncoding::Single((*i).clone()));
            prev = (i, 1);
        }
    }
    res
}

fn main() {
    let v = "aaaabccaadeeee";
    let res = encode_direct(&v.as_bytes());
    println!("{:?}", res);
}