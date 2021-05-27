// (**) Eliminate consecutive duplicates of list elements.

fn main() {
    let v = vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];
    let res = r99p::pack(&v);
    println!("{:?}\n{:?}", v, res);
}