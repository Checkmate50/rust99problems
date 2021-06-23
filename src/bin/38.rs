// (*) Compare the two methods of calculating Euler's totient function.

use std::time::Instant;

fn gcd(n : i32, m : i32) -> i32 {
    let n = n.abs();
    let m = m.abs();
    let mut larger = std::cmp::max(n, m);
    let mut smaller = std::cmp::min(n, m);
    while larger % smaller != 0 {
        let temp = larger % smaller;
        larger = smaller;
        smaller = temp; 
    }
    smaller
}

fn coprime(n : i32, m : i32) -> bool {
    gcd(n, m) == 1
}

fn totient1(n : i32) -> i32 {
    let mut count = 0;
    for i in 1..n {
        if coprime(i, n) {
            count += 1;
        }
    }
    count
}

fn is_prime(n : i32) -> bool {
    for i in 2..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_factors(n: i32) -> Vec<(i32, u32)> {
    let mut num = n;
    let mut res = vec![];
    for i in 2..n {
        if !is_prime(i) {
            continue;
        }
        if num % i == 0 {
            num /= i;
            res.push((i, 1))
        }
        while num % i == 0 {
            num /= i;
            let l = res.len() - 1;
            res[l].1 += 1;
        }
    }
    res
}

fn totient2(n : i32) -> i32 {
    let mut res = 1;
    for (p, m) in prime_factors(n) {
        res *= (p - 1) * p.pow(m-1);
    }
    res
}


fn main() {
    let start = Instant::now();
    println!("{}", totient1(10090));
    let duration = start.elapsed();
    println!("Time elapsed for totient1: {:?}", duration);
    let start = Instant::now();
    println!("{}", totient2(10090));
    let duration = start.elapsed();
    println!("Time elapsed for totient2: {:?}", duration);
}