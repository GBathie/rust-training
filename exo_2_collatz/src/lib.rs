fn collatz_step(x: usize) -> usize {
    if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    }
}

fn collatz_length(mut x: usize) -> usize {
    let mut count = 0;
    while x != 1 {
        x = collatz_step(x);
        count += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collatz_1_is_zero() {
        let x = collatz_length(1);
        assert_eq!(x, 0);
    }

    #[test]
    fn collatz_2() {
        let x = collatz_length(2);
        assert_eq!(x, 1);
    }

    #[test]
    fn collatz_8() {
        let x = collatz_length(8);
        assert_eq!(x, 3);
    }

    #[test]
    fn collatz_27() {
        let x = collatz_length(27);
        assert_eq!(x, 111);
    }
}
