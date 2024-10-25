fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut max_count = 0;
    let mut max_number = 0;

    for i in 1..n + 1 {
        let count = div2count(i);

        if count > max_count {
            max_count = count;
            max_number = i;
        }
    }

    println!("{}", max_number);
}

fn div2count(number: u64) -> u64 {
    let mut number = number;
    let mut count = 0;
    loop {
        count += 1;
        if number % 2 != 0 || number == 1 {
            return count;
        }

        number = number / 2;
    }
}
