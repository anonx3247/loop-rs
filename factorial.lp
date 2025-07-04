fn factorial(n: u64) -> u64 {
	if (n == 0) or (n == 1) {
		1
	} else {
		n * factorial(n - 1)
	}
}

factorial(5)
