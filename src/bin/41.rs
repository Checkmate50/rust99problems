// (**) Given a range of integers by its lower and upper limit, 
// print a list of all even numbers and their Goldbach composition.

fn is_prime(n : u32) -> bool {
    for i in 2..((n as f64).sqrt() as u32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn goldbach(n : u32) -> (u32, u32) {
    for i in 2..n {
        if !is_prime(i) { continue; }
        for j in 2..n {
            if !is_prime(j) { continue; }
            if i + j == n {
                return (i, j);
            }
        }
    }
    panic!("Goldbach doesn't hold for {}!!", n);
}

fn goldbach_list(mut low : u32, high : u32, max : Option<u32>) -> Vec<(u32, u32)> {
    if low <= 2 {
        low = 4;
    }
    if low % 2 == 1 {
        low += 1;
    }
    let mut res = vec![];
    for i in (low..high+1).step_by(2) {
        let (l, r) = goldbach(i);
        if l >= max.unwrap_or(1) {
            res.push((l, r));
        }
    }
    res
}

fn main() {
    println!("{:?}", goldbach_list(9, 20, None));
    println!("{:?}", goldbach_list(1, 2000, Some(50)));
    println!("{}", goldbach_list(2, 3000, Some(50)).len());
}