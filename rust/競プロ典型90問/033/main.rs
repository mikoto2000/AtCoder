fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let h: u32 = FromStr::from_str(&line1[0]).unwrap();
    let w: u32 = FromStr::from_str(&line1[1]).unwrap();

    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        let w_led_count = (w + 1) / 2;
        let h_led_count = (h + 1) / 2;
        println!("{}", w_led_count * h_led_count);
    }
}
