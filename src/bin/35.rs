// Determine the prime factors of a given positive integer. 
// Construct a flat list containing the prime factors in ascending order.

fn is_prime(n : i32) -> bool {
    for i in 2..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_factors(n: i32) -> Vec<i32> {
    let mut num = n;
    let mut res = vec![];
    for i in 2..n {
        if !is_prime(i) {
            continue;
        }
        while num % i == 0 {
            num /= i;
            res.push(i);
        }
    }
    res
}

fn main() {
    println!("{:?}", prime_factors(315));
}