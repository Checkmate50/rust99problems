// (*) Insert an element at a given position into a list.

fn insert_at<T:Clone> (val : T, v : &[T], index : usize) -> Vec<T> {
    let mut res= vec![];
    for i in 0..index-1 {
        res.push(v[i].clone());
    }
    res.push(val);
    for i in index-1..v.len() {
        res.push(v[i].clone());
    }
    res
}

fn main() {
    let s = "abcd";
    let res = insert_at('X' as u8, s.as_bytes(), 2);
    println!("{:?}", std::str::from_utf8(&res));
}