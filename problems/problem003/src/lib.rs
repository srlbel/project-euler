fn largest_prime_factor(mut n: u32) -> u32 {
    let mut f = 2;
    let mut f2 = 1;

    while f * f <= n {
        if n % f == 0 {
            n /= f;
            f2 = f;
        } else {
            f += if f == 2 { 1 } else { 2 }
        }
    }

    if n > 1 { n } else { f2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = largest_prime_factor(13195);
        assert_eq!(result, 29);
    }
}
