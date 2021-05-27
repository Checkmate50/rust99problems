// (**) Replicate the elements of a list a given number of times.

fn repli<T : Clone> (v : &[T], count : i32) -> Vec<T> {
    let mut res = vec![];
    for item in v {
        for _ in 0..count {
            res.push(item.clone())
        }
    }
    res
}

fn main() {
    let mut v = vec![1, 2, 3];
    let res = repli(&mut v, 3);
    println!("{:?}", res);
}