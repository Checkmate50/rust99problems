// (**) Calculate Euler's totient function phi(m).

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

fn totient(n : i32) -> i32 {
    let mut count = 0;
    for i in 1..n {
        if coprime(i, n) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", totient(10));
}