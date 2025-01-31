pub fn fib(n: usize) -> u64 {
    let mut memo = vec![0; n + 1];
    memo[1] = 1;

    for i in 2..=n {
        memo[i] = memo[i - 1] + memo[i - 2];
    }

    memo[n]
}
