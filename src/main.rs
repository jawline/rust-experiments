mod sieve;
mod fibs;
mod fact;

fn main() {

	assert_eq!(8, fibs::fibs(6));

	for i in 1..10 {
		assert_eq!(fact::fact(i), fact::fact2(i));
	}

	for i in 1..10 {
		assert_eq!(fact::fact(i), fact::fact3(i));
	}

	println!("Fact Passed");

	for i in sieve::primes(10000) {
		assert!(sieve::is_prime(i))
	}

	println!("Sieve Passed");

	let q = sieve::sieve(100);

	for i in 1..100 {
		assert_eq!(q[i], sieve::is_prime(i));
	}

	println!("Sieve = is_prime passed");

	for i in 0..40 {
		assert_eq!(fibs::fibs(i), fibs::fibs2(i));
		assert_eq!(fibs::fibs(i), fibs::fibs3(i));
	}

	println!("Fibs = Fibs2 pass");
	println!("Fibs = Fibs3 pass");
}