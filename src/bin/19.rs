// (**) Rotate a list N places to the left.

fn rotate<T:Clone> (v : &[T], amount : i32) -> Vec<T> {
    let mut result = vec![];
    let trimmed = amount % (v.len() as i32);
    if trimmed >= 0 {
        for i in 0..((v.len() as i32) - trimmed) {
            result.push(v[(i + trimmed) as usize].clone())
        }
        for i in 0..trimmed {
            result.push(v[i as usize].clone());
        }
    } else {
        for i in 0..(-trimmed) {
            result.push(v[((v.len() as i32) + trimmed + i) as usize].clone());
        }
        for i in 0..((v.len() as i32) + trimmed) {
            result.push(v[i as usize].clone())
        }
    }
    result
}

fn main() {
    let s = "abcdefghik";
    let res = rotate(&s.as_bytes(), 3);
    let res2 = rotate(&s.as_bytes(), -2);
    println!("{:?}\n{:?}", std::str::from_utf8(&res), std::str::from_utf8(&res2));
}