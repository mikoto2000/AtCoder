fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let k: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let a: u64 = FromStr::from_str(&line2[0]).unwrap();
    let b: u64 = FromStr::from_str(&line2[1]).unwrap();

    // 10 進数へ変換
    let ad = ktod(k, a);
    let bd = ktod(k, b);

    println!("{}", ad * bd);
}

fn ktod(k: u64, k_number: u64) -> u64 {
    let mut k_number = k_number;
    let mut k_digit = 1;
    let mut dec_number = 0;

    while k_number > 0 {
        let target_digit_number = k_number % 10;
        dec_number += target_digit_number * k_digit;

        k_digit *= k;
        k_number /= 10;
    }

    return dec_number;
}
