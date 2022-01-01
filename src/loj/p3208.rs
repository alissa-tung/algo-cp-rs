use std::{
  fs::{self, File},
  io::Write,
};

pub fn main() {
  let buf = fs::read_to_string("code.in").unwrap();
  let args: [usize; 2] = buf
    .split_ascii_whitespace()
    .take(2)
    .map(|x| x.parse().unwrap())
    .collect::<Vec<usize>>()
    .try_into()
    .unwrap();

  let xs = {
    let xs = with_strlen(format!("{:b}", args[1]), args[0])
      .as_bytes()
      .to_vec();
    let mut ys: Vec<u8> = vec![b'0'; args[0]];
    ys[0] = xs[0];
    for i in 1..xs.len() {
      ys[i] = xor(xs[i - 1], xs[i]);
    }
    ys
  };

  let mut fw = File::create("code.out").unwrap();
  unsafe {
    let xs = String::from_utf8_unchecked(xs);
    fw.write_all(xs.as_bytes()).unwrap();
  }
}

fn with_strlen(xs: String, n: usize) -> String {
  let len = xs.len();
  let ys = "0".repeat(n - len);
  format!("{}{}", ys, xs)
}

fn xor(x: u8, y: u8) -> u8 {
  if x == y {
    b'0'
  } else {
    b'1'
  }
}
