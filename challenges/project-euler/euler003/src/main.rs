/*
Problem 3 - Largest prime factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
 */

fn main() {
    let to_be_factored: u64 = 600851475143;
    let primes = find_all_primes_smaller_than(f64::sqrt(to_be_factored as f64) as u64);
    for prime in primes.iter().rev() {
        if to_be_factored % prime == 0 {
            println!("Found largest prime factor: {}", prime);
            break;
        }
    }
}

/*
algorithm Sieve of Eratosthenes
    input: an integer n > 1.
    output: all prime numbers from 2 through n.

    let A be an array of Boolean values, indexed by integers 2 to n,
    initially all set to true.

    for i = 2, 3, 4, ..., not exceeding √n do
        if A[i] is true
            for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
                set A[j] := false

    return all i such that A[i] is true.
*/

/// Finds all prime numbers smaller than given n.
///
/// Implements a very poorly written (I have been coding in Rust for 2 hours and I am not sure of its correctness) Sieve of Eratosthenes.
fn find_all_primes_smaller_than(n: u64) -> Vec<u64> {
    let mut a = vec![true; n.try_into().unwrap()];

    for i in 2..(f32::sqrt(n as f32)) as usize {
        if a[i] {
            for j in (2 * i..n as usize).step_by(i) {
                a[j] = false;
            }
        }
    }
    let mut primes: Vec<u64> = vec![];
    for i in 2..n as usize {
        if a[i] {
            primes.push(i.try_into().unwrap());
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let to_be_factored: u64 = 13195;
        let primes = find_all_primes_smaller_than(f64::sqrt(to_be_factored as f64) as u64);
        for prime in primes.iter().rev() {
            if to_be_factored % prime == 0 {
                assert_eq!(prime, &29);
                break;
            }
        }
    }

    #[test]
    fn test_first_primes() {
        assert_eq!(vec![2, 3, 5, 7, 9], find_all_primes_smaller_than(10));
    }
}
