fn main() {
    is_prime(28);
}

fn is_prime(n: u32) -> bool {
    // TODO: return true only if n is prime.
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
