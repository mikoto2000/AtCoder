use std::str::FromStr;
fn main() {
    let mut card_number = vec![vec![0; 3]; 3];
    let mut card_open = vec![vec![false; 3]; 3];

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    card_number[0][0] = FromStr::from_str(&line1[0]).unwrap();
    card_number[0][1] = FromStr::from_str(&line1[1]).unwrap();
    card_number[0][2] = FromStr::from_str(&line1[2]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    card_number[1][0] = FromStr::from_str(&line2[0]).unwrap();
    card_number[1][1] = FromStr::from_str(&line2[1]).unwrap();
    card_number[1][2] = FromStr::from_str(&line2[2]).unwrap();

    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    line3 = line3.trim().parse::<String>().ok().unwrap();
    let line3: Vec<&str> = line3.split_whitespace().collect();
    card_number[2][0] = FromStr::from_str(&line3[0]).unwrap();
    card_number[2][1] = FromStr::from_str(&line3[1]).unwrap();
    card_number[2][2] = FromStr::from_str(&line3[2]).unwrap();

    let mut line4 = String::new();
    std::io::stdin().read_line(&mut line4).ok();
    line4 = line4.trim().parse::<String>().ok().unwrap();
    let line4: Vec<&str> = line4.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line4[0]).unwrap();

    for _ in 0..n {
        let mut line5 = String::new();
        std::io::stdin().read_line(&mut line5).ok();
        line5 = line5.trim().parse::<String>().ok().unwrap();
        let line5: Vec<&str> = line5.split_whitespace().collect();
        let b: u64 = FromStr::from_str(&line5[0]).unwrap();

        open(b, &card_number, &mut card_open);
    }

    if is_bingo(&card_open) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

fn open(number: u64, card_number: &Vec<Vec<u64>>, card_open: &mut Vec<Vec<bool>>) {
    for j in 0..3 {
        for k in 0..3 {
            if card_number[j][k] == number {
                card_open[j][k] = true;
                return;
            }
        }
    }
}

fn is_bingo(card: &Vec<Vec<bool>>) -> bool {
    // 横
    for i in 0..3 {
        if card[i].iter().all(|v| *v) {
            return true;
        }
        // 縦
        if card[0][i] && card[1][i] && card[2][i] {
            return true;
        }
    }

    // 斜め
    if card[0][0] && card[1][1] && card[2][2] {
        return true;
    }
    if card[0][2] && card[1][1] && card[2][0] {
        return true;
    }

    false
}
