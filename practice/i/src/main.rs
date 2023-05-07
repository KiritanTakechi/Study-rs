fn main() {
    let number: i64 = 1234;
    let ans = count_bits(number);
    println!("{:?}",ans)
}
fn count_bits(n: i64) -> u32 {
    let mut num = n.clone();
    let mut count: u32 = 0;
    while num > 0 {
        count += (num & 1) as u32;
        num >>= 1;
    }
    count
}
