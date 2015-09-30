fn extract_from_sieve(results: Vec<bool>) -> Vec<usize> {
	results.iter().enumerate().fold(Vec::new(), |mut result, (i, &v)| {
		if v {
			result.push(i);
		}
		result
	})
}

pub fn sieve(n: usize) -> Vec<bool> {

	let mut results: Vec<bool> = (0..n).map(|i| match i {
		0 | 1 => false,
		_ => true
	}).collect();

	for i in 2..n {
		for q in i+1..n {
			if q % i == 0 {
				results[q] = false;
			}
		}
	}

	results
}

pub fn primes(n: usize) -> Vec<usize> {
	extract_from_sieve(sieve(n))
}

pub fn is_prime(n: usize) -> bool {
	match n {
		0 | 1 => false,
		_ => (2..(n/2) + 1).find(|x| n % x == 0).is_none()
	}
}