// (**) Goldbach's conjecture.

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

fn main() {
    println!("{:?}", goldbach(28));
}