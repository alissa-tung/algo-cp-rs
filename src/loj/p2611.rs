use std::io;
#[allow(dead_code)]

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    buf = "".to_string();
    io::stdin().read_line(&mut buf).unwrap();
    let xs = buf
        .split_ascii_whitespace()
        .take(n)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut acc: u64 = 0;
    let mut ys: Vec<u32> = vec![];
    for x in xs.iter() {
        if ys.is_empty() || ys.last().unwrap() < x {
            ys.push(*x)
        } else if ys.last().unwrap() == x {
        } else {
            acc += (ys.last().unwrap() - x) as u64;
            ys = vec![*x];
        }
    }
    acc += *ys.last().unwrap() as u64;
    println!("{}", acc)
}
