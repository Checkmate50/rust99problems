// (**) Generate the combinations of K distinct objects chosen from the N elements of a list

fn combinations<T:Clone>(v : &[T], count : usize) -> Vec<Vec<T>> {
    let mut res = vec![];
    if count == 0 || v.len() == 0 {
        return res;
    }
    if count == 1 {
        for i in v {
            res.push(vec![i.clone()]);
        }
        return res;
    }
    for i in 0..v.len() {
        for vc in combinations(&v[i+1..v.len()], count-1) {
            let mut next = vec![];
            next.push(v[i].clone());
            for vci in vc {
                next.push(vci);
            }
            res.push(next);
        }
    }
    res
}

fn main() {
    let s = "abcdef";
    let res = combinations(s.as_bytes(), 3);
    println!("{:?}", res.into_iter().map(|x| String::from_utf8(x).unwrap()).collect::<Vec<_>>());
}