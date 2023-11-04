using Primes
compute_min_sum(x) = prod(primes(x).^[Int(floor(log(b,x))) for b in primes(x)])
compute_min_sum(20)