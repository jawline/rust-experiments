pub fn fact2(n: i64) -> i64 {
	match n {
		1 => 1,
		_ => n * fact(n-1)
	}
}

pub fn fact3(n: i64) -> i64 {
	let mut q = n;
	for i in (1..n).rev() {
		q = q * i;
	}
	q
}

pub fn fact(n: i64) -> i64 {
	(1..n).rev().fold(n, |curr, next| curr * next)
}