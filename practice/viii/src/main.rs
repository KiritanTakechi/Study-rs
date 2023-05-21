fn main() {
    println!("Hello, world!");
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    for int in ints {
        if ints.contains(&(s - int)) && (ints.iter().count(s) > 1 || (s - int != *int)) {
            return (int, s - int);
        }
    }
    return None;
}