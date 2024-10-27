fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();

    // vec にしてソートして join する
    let mut chars: Vec<_> = line1.chars().collect();
    chars.sort();
    let result: String = chars.into_iter().collect();


    if result == "ABC" {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
