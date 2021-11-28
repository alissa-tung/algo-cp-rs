use std::io;

#[allow(dead_code)]

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let args: [usize; 2] = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let mut ys: Vec<i64> = vec![0; args[0] + 1];

    buf = "".to_string();
    io::stdin().read_line(&mut buf).unwrap();
    let _ = (1..=args[0])
        .zip(buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()))
        .map(|(fst, snd)| {
            let mut fst = fst;
            while fst <= args[0] {
                ys[fst] += snd as i64;
                fst += lowbit(fst as i32);
            }
        })
        .collect::<Vec<()>>();

    for _ in 0..args[1] {
        buf = "".to_string();
        io::stdin().read_line(&mut buf).unwrap();
        let xs: [i32; 3] = buf
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        if xs[0] == 1 {
            let mut x = xs[1] as usize;
            while x <= args[0] {
                ys[x] += xs[2] as i64;
                x += lowbit(x as i32);
            }
        } else {
            let mut acc = 0;
            let mut x = xs[2] as usize;
            while 0 < x {
                acc += ys[x];
                x -= lowbit(x as i32);
            }
            x = xs[1] as usize - 1;
            while 0 < x {
                acc -= ys[x];
                x -= lowbit(x as i32);
            }
            println!("{}", acc)
        }
    }
}

fn lowbit(x: i32) -> usize {
    (x & -x) as usize
}
