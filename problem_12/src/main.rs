struct PrimeFacCalc {
    prev_primes: Vec<u32>,
    prev_prime_check: u32
}

impl PrimeFacCalc {
    fn new() -> PrimeFacCalc{
        return PrimeFacCalc{ prev_primes: Vec::new(), prev_prime_check: 2 };
    }
    fn calculate_primes_to(&mut self, to: u32) {
        // println!("primes to {}",to);
        if to <= self.prev_prime_check {
            return;
        }
        for pp in self.prev_prime_check..to+1 {
            if is_prime(pp, &self.prev_primes) {
                self.prev_primes.push(pp);
            }
        }
        self.prev_prime_check = to+1;
    }
    fn get_fac_count(&mut self, number: u32) -> u32 {
        let mut cnum = number;
        let mut fac_count = 1;
        self.calculate_primes_to(number/2);
        // println!("prev_primes {:?}",&self.prev_primes);
        for prime in &self.prev_primes {
            let mut pfcount = 1;
            while cnum % prime == 0 {
                cnum = cnum / prime;
                pfcount = pfcount + 1;
            }
            fac_count = fac_count * pfcount;
        }
        if cnum != 1 {
            fac_count = fac_count + 1;
        }
        return fac_count;
    }
}

fn is_prime(pp: u32, prev: &Vec<u32>) -> bool {
    // println!("{}",pp);
    let bound = (pp as f64).sqrt() as u32;
    // println!("bound={}",bound);
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

fn main() {
    let mut csum = 0;
    let mut cnum = 1;
    let mut pfc = PrimeFacCalc::new();
    let mut last_fac_count = 0;
    while last_fac_count < 500 {
        csum = csum + cnum;
        cnum = cnum + 1;
        if csum % 2 == 0 && csum % 3 == 0 && csum % 5 == 0  && csum % 7 == 0 && csum % 11 == 0 {
            last_fac_count = pfc.get_fac_count(csum);
            println!("{} - {}",csum,last_fac_count);
        }
        // csum = 6;
        // break;
    }
    println!("csum = {}",csum);
}
