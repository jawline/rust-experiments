pub fn fibs3(n: usize) -> u64 {
	match n {
		0 => 0,
		1 => 1,
		_ => fibs(n-1) + fibs(n-2)
	}
}

pub fn fibs(n: usize) -> u64 {
	match n {
		0 => 0,
		1 => 1,
		_ => (1..n).fold((0, 1), |(l1, l2), _| (l2, l1 + l2)).1
	}
}

pub fn fibs2(n: usize) -> u64 {
	match n {
		0 => 0,
		1 => 1,
		_ => {
			let mut l2 = 0;
			let mut l1 = 1;

			for _ in 2..n {
				let curr = l1 + l2;
				l2 = l1;
				l1 = curr;
			}
			
			l1 + l2
		}
	}
}