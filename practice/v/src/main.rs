fn main() {
    let sec: u32 = 0;
    println!("{}", make_readable(sec))
}

fn make_readable(seconds: u32) -> String {
    format!(
        "{:0>2}:{:0>2}:{:0>2}",
        seconds / 3600,
        seconds % 3600 / 60,
        seconds % 60
    )
}
