// (**) Determine whether a given integer number is prime.

fn gcd(mut n : i32, mut m : i32) -> i32 {
    n = n.abs();
    m = m.abs();
    let mut larger = std::cmp::max(n, m);
    let mut smaller = std::cmp::min(n, m);
    while larger % smaller != 0 {
        let temp = larger % smaller;
        larger = smaller;
        smaller = temp; 
    }
    smaller
}

fn main() {
    println!("{} {} {}", gcd(36, 63), gcd(-3, -6), gcd(-3, 6));
}