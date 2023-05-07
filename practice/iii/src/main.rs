fn main() {
    let num: i64 = 42;
    println!("{}", row_sum_odd_numbers(num))
}
fn row_sum_odd_numbers(n: i64) -> i64 {
    let first: i64 = 1 + n * (n - 1);
    let second: i64 = first + (n - 1) * 2;
    (first + second) * n / 2
}
