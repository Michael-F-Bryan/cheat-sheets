#![feature(inclusive_range_syntax)]
#![feature(step_by)]
#![crate_type = "cdylib"]

#[no_mangle]
pub extern "C" fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n if n & 1 == 0 => false,   // even numbers
        n => {
            let limit = (n as f32).sqrt() as u32 + 1;
            for i in (4...limit).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}

#[no_mangle]
pub extern "C" fn spare() {
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_tester_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
        for prime in primes {
            assert!(is_prime(prime));
        }

        let not_primes = vec![0, 1, 4, 10, 22];
        for num in not_primes {
            assert!(!is_prime(num));
        }
    }
}
