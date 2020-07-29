pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    if n <= 1 {
        return n;
    }

    for _i in 2..n + 1 {
        let c = a + b;
        a = b;
        b = c;
    }

    return b;
}
