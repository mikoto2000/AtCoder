fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let t = line2.trim().parse::<String>().ok().unwrap();
    let t2 = t.clone() + &t;

    for i in 0..s.len() {
        if s == t2[i..i + s.len()] {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
