// (**) Determine whether a given integer number is prime.

fn is_prime(n : i32) -> bool {
    for i in 2..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{} {}", is_prime(7), is_prime(12));
}