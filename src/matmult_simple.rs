fn fib(n: u32) -> u32 {
    let mut fib = [0, 1, 1, 1];

    let mut result = [1, 0, 0, 1]; 

    let mut n = n;
    while n > 0 {
        
        let temp = [
            result[0] * fib[0] + result[1] * fib[2],
            result[0] * fib[1] + result[1] * fib[3],
            result[2] * fib[0] + result[3] * fib[2],
            result[2] * fib[1] + result[3] * fib[3],
        ];
        
        result = temp;

        n -= 1;
    }

    result[0]
}