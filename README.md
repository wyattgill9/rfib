## Naïve Implementation

The function follows the basic definition of Fibonacci numbers:

- **Base Cases:**
  - `fib(0) = 0`
  - `fib(1) = 1`
- **Recursive Case:**
  - `fib(n) = fib(n - 1) + fib(n - 2)`

### Code

```rust
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

## Complexity

- **Time Complexity:** O(2^n) (EXP growth)
- **Space Complexity:** O(n) (Call stack depth)

## Usage

Compile and run using Rust:

```sh
cargo run --release
```

## License

This project is open-source and available under the MIT License.

