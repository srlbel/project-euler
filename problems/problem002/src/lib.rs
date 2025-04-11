fn fibo_even_sum(n: usize) -> u32 {
    let mut fib = vec![1, 2];

    for i in 0..n {
        let next = fib[i] + fib[i + 1];
        fib.push(next);
    }

    fib.into_iter().filter(|x| x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibo_even_sum(8);
        assert_eq!(result, 10);
    }
}
