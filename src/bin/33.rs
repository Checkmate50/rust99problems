// (*) Determine whether two positive integer numbers are coprime. 
// Two numbers are coprime if their greatest common divisor equals 1

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

fn main() {
    println!("{}", coprime(35, 64));
}