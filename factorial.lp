-- Making a factorial program

n := 10

if n == 0 or n == 1 {
	1
} else {
	mut k := 1
	mut m := 1
	while k <= n {
		m := m * k
		k := k + 1
	}
	m
}
