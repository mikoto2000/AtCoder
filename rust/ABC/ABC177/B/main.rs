fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();
    let s_len = s.len();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let t = line2.trim().parse::<String>().ok().unwrap();
    let t_len = t.len();

    let mut min_count = 1000;
    for i in 0..(s_len - t_len + 1) {
        let i = i as usize;
        let mut count = 0;
        //let srange = &s[i..i + t_len];
        //println!("{}", srange);
        for j in 0..t_len {
            let j = j as usize;
            let tt = &t[(j)..(j + 1)];
            let ts = &s[(i + j)..(i + j + 1)];
            //println!("{} {}", tt, ts);
            if tt != ts {
                count += 1;
            }
        }
        //println!("{}", count);
        if count < min_count {
            min_count = count;
        }
    }

    println!("{}", min_count);
}
