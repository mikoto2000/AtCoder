use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut a: Vec<u64> = line2.split_whitespace().map(|v| FromStr::from_str(&v).unwrap()).collect();

    let mut alice_score = 0;
    let mut bob_score = 0;

    loop {
        // alice がカードを引く
        if a.len() == 0 {
            break;
        }
        let a_max = a.iter().max().unwrap();
        alice_score += a_max;
        let acp = a.iter().position(|e| *e == *a_max).unwrap();
        a.remove(acp);

        // bob
        if a.len() == 0 {
            break;
        }
        let b_max = a.iter().max().unwrap();
        bob_score += b_max;
        let bcp = a.iter().position(|e| *e == *b_max).unwrap();
        a.remove(bcp);
    }

    println!("{}", alice_score - bob_score);
}
