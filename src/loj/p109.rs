use std::io;
#[allow(dead_code)]

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let args: [u32; 2] = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    let mut p: Vec<u32> = (0..args[0]).collect();
    let mut acc: u32 = 0;

    for i in 0..args[1] {
        buf = "".to_string();
        io::stdin().read_line(&mut buf).unwrap();
        let xs: [u32; 3] = buf
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        if xs[0] == 1 {
            let x = find(&mut p, xs[1]);
            let y = find(&mut p, xs[2]);
            if x == y {
                acc = (acc + 1) % 998244353;
            }
            if i != args[1] - 1 {
                acc = (acc << 1) % 998244353;
            }
        } else {
            union(&mut p, xs[1], xs[2])
        };
    }

    println!("{}", &acc)
}

fn find(xs: &mut Vec<u32>, x: u32) -> u32 {
    let p = xs[x as usize];
    if p != x {
        xs[x as usize] = find(xs, p);
        xs[x as usize]
    } else {
        p
    }
}

fn union(xs: &mut Vec<u32>, x: u32, y: u32) {
    let x = find(xs, x);
    let y = find(xs, y);
    if x != y {
        xs[y as usize] = x
    }
}
