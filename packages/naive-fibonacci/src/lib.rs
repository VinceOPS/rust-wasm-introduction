use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Naive implementation (recursive) of the fibonacci sequence.
/// Returns a specific Fibonacci number `Fn` for the given `n`.
pub fn naive_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    return naive_fibonacci(n - 1) + naive_fibonacci(n - 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_0_for_input_0() {
        assert_eq!(naive_fibonacci(0), 0);
    }

    #[test]
    fn it_returns_1_for_input_1() {
        assert_eq!(naive_fibonacci(1), 1);
    }

    #[test]
    fn it_returns_55_for_input_10() {
        assert_eq!(naive_fibonacci(10), 55);
    }

    #[test]
    fn it_returns_832040_for_input_30() {
        assert_eq!(naive_fibonacci(30), 832040);
    }
}
