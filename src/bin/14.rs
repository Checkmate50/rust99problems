// (*) Duplicate the elements of a list.

fn dupli<T : Clone> (v : &[T]) -> Vec<T> {
    let mut res = vec![];
    for i in v {
        res.push(i.clone());
        res.push(i.clone());
    }
    res
}

fn main() {
    let mut v = vec![1, 2, 3];
    let res = dupli(&mut v);
    println!("{:?}", res);
}