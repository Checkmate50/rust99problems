// (**) Calculate Euler's totient function phi(m) (improved).

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

fn totient(n : i32) -> i32 {
    let mut res = 1;
    for (p, m) in prime_factors(n) {
        res *= (p - 1) * p.pow(m-1);
    }
    res
}


fn main() {
    println!("{}", totient(10));
}