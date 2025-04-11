fn largest_palindrome_product(n: u32) -> u32 {
    let lower: u32 = 10_u32.pow(n - 1);
    let upper: u32 = 10_u32.pow(n) - 1;
    let mut largest = 0;

    for i in (lower..=upper).rev() {
        for j in (lower..=upper).rev() {
            let product = i * j;
            if product <= largest {
                break;
            }

            let s = product.to_string();
            if s.chars().eq(s.chars().rev()) {
                largest = product;
            }
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = largest_palindrome_product(3);
        assert_eq!(result, 906609);
    }

    #[test]
    fn it_works_2() {
        let result = largest_palindrome_product(2);
        assert_eq!(result, 9009);
    }
}
