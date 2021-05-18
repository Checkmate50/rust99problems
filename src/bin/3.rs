// Find kth element of a list

fn my_but_last<T>(v : &[T], i : usize) -> Option<&T> {
    if v.len() < i {
        return None;
    }
    Some(&v[i-1])
}

fn main() {
    let v = vec![1, 2, 3, 4];
    r99p::print_option(my_but_last(&v, 2));
    
    let v2 = vec!['x', 'y', 'z'];
    r99p::print_option(my_but_last(&v2, 3));
}