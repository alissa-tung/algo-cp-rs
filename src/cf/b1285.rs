use std::io;

pub fn main() {
  let mut buf = String::new();
  let argc: usize = {
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
  };

  for _ in 0..argc {
    let argc: usize = {
      buf.clear();
      io::stdin().read_line(&mut buf).unwrap();
      buf.trim().parse().unwrap()
    };

    let args: Vec<i32> = {
      buf.clear();
      io::stdin().read_line(&mut buf).unwrap();
      buf
        .split_ascii_whitespace()
        .take(argc)
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
    };

    println!("{}", if solve(argc, &args) { "YES" } else { "NO" })
  }
}

#[allow(clippy::needless_range_loop)]
fn solve(argc: usize, args: &[i32]) -> bool {
  let mut acc = 0_i64;
  for i in 0..argc {
    acc += args[i] as i64;
    if acc <= 0 {
      return false;
    }
  }

  acc = 0_i64;
  for i in 0..argc {
    let i = argc - i - 1;
    acc += args[i] as i64;
    if acc <= 0 {
      return false;
    }
  }

  true
}
