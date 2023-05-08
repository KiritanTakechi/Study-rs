fn main() {
    let num = 30_u64;
    println!("{}", zeros(num));
}

fn zeros(n: u64) -> u64 {
    let mut i = 5;
    let mut ans = 0;
    while i <= n {
        ans += n / i;
        i *= 5;
    }
    ans
}
