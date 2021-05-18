// Reverse a list.

fn reverse<T>(v : &mut Vec<T>) {
    for i in 0..v.len() / 2 {
        let j = v.len() - i - 1;
        v.swap(i, j);
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4];
    reverse(&mut v);
    println!("{:?}", v);
    
    let mut v2 = vec!['x', 'y', 'z'];
    reverse(&mut v2);
    println!("{:?}", v2);
}