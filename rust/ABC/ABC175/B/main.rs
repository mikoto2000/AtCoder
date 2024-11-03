use std::str::FromStr;

fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let n = line1.trim().parse::<usize>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut ls: Vec<i128> = line2
        .split_whitespace()
        .map(|v| <i128 as FromStr>::from_str(&v).unwrap())
        .collect();

    // そもそも辺が足りない
    if ls.len() < 3 {
        println!("0");
        return;
    }

    ls.sort();

    let mut count = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            // 長さが異なるか？
            if ls[i] == ls[j] {
                continue;
            }
            for k in j + 1..n {
                // 長さが異なるか？
                if ls[i] == ls[k] || ls[j] == ls[k] {
                    continue;
                }

                // 三角形がつくれるか？
                if ls[k] < ls[i] + ls[j] {
                    count += 1;
                } else {
                    // ソートしているから、
                    // これ以上進めても三角形がつくれることはない。
                    break;
                }
            }
        }
    }
    println!("{}", count);
}
