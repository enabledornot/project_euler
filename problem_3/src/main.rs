fn main() {
    println!("Largest Prime Factor of 13195 is {}",largest_prime_factor(13195));
    println!("Largest Prime Factor of 600851475143 is {}",largest_prime_factor(600851475143));
}
fn first_n_primes(n: i32) -> Vec<i32> {
    let mut primes = vec![];
    let mut potential_prime = 2;
    loop {
        if primes.len()==n as usize {
            return primes;
        }
        if primes.iter().all(|&x| potential_prime%x!=0) {
            primes.push(potential_prime);
        }
        potential_prime+=1;
    }
}
fn largest_prime_factor(mut number: u128) -> u128 {
    let mut primes = vec![];
    let mut potential_prime = 2;
    loop {
        if primes.iter().all(|&x| potential_prime%x!=0) {
            primes.push(potential_prime);
            while number%potential_prime == 0 {
                number/=potential_prime;
            }
            if number==1 {
                return potential_prime;
            }
        }
        potential_prime+=1;
    }
}