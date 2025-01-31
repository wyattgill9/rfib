pub fn fib(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}