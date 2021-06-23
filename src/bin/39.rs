// (*) A list of prime numbers.

fn is_prime(n : u32) -> bool {
    for i in 2..((n as f64).sqrt() as u32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_range(low : u32, high : u32) -> Vec<u32> {
    let mut res = vec![];
    for i in low..high+1 {
        if is_prime(i) {
            res.push(i);
        }
    }
    res
}

fn main() {
    println!("{:?}", prime_range(10, 20));
}