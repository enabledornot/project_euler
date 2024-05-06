fn main() {
    println!("The sum of primes below 2,000,000 is {}",sum_prime_below(2000000));
}
fn is_prime(pp: u64, prev: &Vec<u64>) -> bool {
    let bound = (pp as f64).sqrt() as u64;
    // println!("bound = {}",bound);
    for op in prev {
        if *op > bound {
            return true;
        }
        if pp % *op == 0 {
            return false;
        }
    }
    return true;
}
fn sum_prime_below(amnt: u64) -> u64 {
    let mut potential_prime: u64 = 2;
    let mut prev_primes: Vec<u64> = Vec::new();
    while potential_prime < amnt {
        if is_prime(potential_prime,&prev_primes) {
            prev_primes.push(potential_prime);
        }
        potential_prime = potential_prime + 1;
    }
    // println!("prev_primes = {:?}",prev_primes);
    return prev_primes.iter().sum();
}