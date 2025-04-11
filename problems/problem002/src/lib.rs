fn fibo_even_sum(n: u32) -> u32 {
    let (mut a, mut b) = (1, 2);
    let mut acc: u32 = 0;

    while b <= n {
        if b % 2 == 0 {
            acc += b;
        }

        let next = a + b;
        a = b;
        b = next;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibo_even_sum(60);
        assert_eq!(result, 44);
    }
}
