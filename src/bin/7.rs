// (**) Flatten a nested list structure.

fn my_flatten<T>(v : &[impl PartialEq]) -> bool {
    for i in 0..v.len() / 2 {
        if v[i] != v[v.len() - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let v = vec![1,2,3];
    println!("{}", is_palindrome(&v));
    
    let s = "madamimadam";
    println!("{}", is_palindrome(&s.as_bytes()));

    let v2 = vec![1,2,4,8,16,8,4,2,1];
    println!("{}", is_palindrome(&v2))
}