// Find the number of elements of a list.

fn my_length<T>(v : &[T]) -> usize {
    v.len()
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", my_length(&v));
    
    let v2 = vec!['x', 'y', 'z'];
    println!("{}", my_length(&v2));
}