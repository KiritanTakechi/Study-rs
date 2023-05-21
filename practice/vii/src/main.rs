fn main() {
    println!("{}",print(3).unwrap());

}

fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}

fn prints(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 {
        return None;
    }

    let mut diamond = String::new();
    let middle = n / 2;

    for i in 0..n {
        let spaces = if i <= middle { middle - i } else { i - middle };
        let asterisks = n - 2 * spaces;

        diamond.push_str(&" ".repeat(spaces as usize));
        diamond.push_str(&"*".repeat(asterisks as usize));
        diamond.push_str("\n");
    }

    Some(diamond)
}