fn main() {
    let name = [];
    println!("{}", likes(&name))
}

fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, _, ..] => format!("{}, {} and {} others like this", a, b, names.len() - 2),
    }
}