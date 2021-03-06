// Find the last element of a list.

fn my_last<T>(v : &[T]) -> Option<&T> {
    if v.len() == 0 {
        return None;
    }
    Some(&v[v.len()-1])
}

fn main() {
    let v = vec![1, 2, 3, 4];
    r99p::print_option(my_last(&v));
    
    let v2 = vec!['x', 'y', 'z'];
    r99p::print_option(my_last(&v2));
}