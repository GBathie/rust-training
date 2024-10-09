fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

#[cfg(test)]
mod test {
    use super::fib;

    #[test]
    fn fib0() {
        let x = fib(0);
        assert_eq!(x, 0);
    }

    #[test]
    fn fib1() {
        let x = fib(1);
        assert_eq!(x, 1);
    }

    #[test]
    fn fib2() {
        let x = fib(2);
        assert_eq!(x, 1);
    }

    #[test]
    fn fib3() {
        let x = fib(3);
        assert_eq!(x, 2);
    }

    #[test]
    fn fib4() {
        let x = fib(4);
        assert_eq!(x, 3);
    }

    #[test]
    fn fib5() {
        let x = fib(5);
        assert_eq!(x, 5);
    }

    #[test]
    fn fib15() {
        let x = fib(15);
        assert_eq!(x, 610);
    }
}
