fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let o = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let e = line2.trim().parse::<String>().ok().unwrap();

    for i in 0..o.len() {
        let range_max = i + 1;
        let o_str = &o[i..range_max];
        print!("{}", o_str);

        if i < e.len() {
            let e_str = &e[i..range_max];
            print!("{}", e_str);
        }
    }
    println!();
}
