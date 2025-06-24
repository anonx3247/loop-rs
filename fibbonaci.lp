-- Fibbonaci Sequence

mut a := 0
mut b := 1

while b < 1000 {
	mut t := a + b
	a := b
	b := t
	b
}
