fn gdc(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gdc(b, a % b)
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    a * b / gdc(a, b)
}

fn smallest_mult(n: u32) -> u32 {
    (1..=n).fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = smallest_mult(10);
        assert_eq!(result, 2520);
    }

    #[test]
    fn it_works_2() {
        let result = smallest_mult(13);
        assert_eq!(result, 360360);
    }
}
