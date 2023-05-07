fn main() {
    println!("{}", get_middle("strr"));
}

fn get_middle<'a>(s: &'a str) -> &'a str {
    let len = s.len();
    let mid = len >> 1;
    if len & 1 == 1 {
        &s[mid..=mid]
    } else {
        &s[mid - 1..=mid]
    }
}
