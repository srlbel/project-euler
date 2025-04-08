fn multiples_of3_or5(n: i32) -> i32 {
    let mut acc = 0;

    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            acc += i;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiples_of3_or5(49);
        assert_eq!(result, 543);
    }
}
