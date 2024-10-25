use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u128 = FromStr::from_str(&line1[0]).unwrap();
    let k: u128 = FromStr::from_str(&line1[1]).unwrap();

    let mut result = n;
    for _ in 0..k {
        result = process(result);
    };

    println!("{}", result);
}

fn process(number: u128) -> u128 {
    // 8 進数を 9 進数に直す
    let n_number = e2n(number);

    // 8 を 5 に置き換える
    FromStr::from_str(&n_number.to_string().replace("8", "5")).unwrap()
}

fn e2n(number: u128) -> u128 {
    let dec = ktod(8, number);
    dtok(9, dec)
}

fn ktod(k: u128, k_number: u128) -> u128 {
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

fn dtok(k: u128, d_number: u128) -> u128 {
    let mut d_number = d_number;
    let mut d_digit = 1;
    let mut k_number = 0;

    while d_number > 0 {
        let target_digit_number = d_number % k;
        k_number += target_digit_number * d_digit;

        d_digit *= 10;
        d_number /= k;
    }

    return k_number;
}
