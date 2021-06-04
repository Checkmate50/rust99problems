// (*) Remove the K'th element from a list.

fn remove_at<T:Clone> (v : &[T], index : usize) -> (T, Vec<T>) {
    let ir = v[index-1].clone();
    let mut res= vec![];
    for i in 0..index-1 {
        res.push(v[i].clone());
    }
    for i in index..v.len() {
        res.push(v[i].clone());
    }
    (ir, res)
}

fn main() {
    let s = "abcd";
    let res = remove_at(s.as_bytes(), 2);
    println!("({}, {:?})", res.0, res.1);
}