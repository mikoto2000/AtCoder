fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().to_string();

    if line1 == "Hello,World!".to_string() {
        println!("{}", "AC");
    } else {
        println!("{}", "WA");
    }
}
