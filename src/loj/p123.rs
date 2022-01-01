use std::{cmp::Reverse, collections::BinaryHeap, convert::TryInto, io};

pub fn _main() {
  let mut buf = String::new();
  io::stdin().read_line(&mut buf).unwrap();
  let args: [usize; 2] = buf
    .split_ascii_whitespace()
    .take(2)
    .map(|x| x.parse().unwrap())
    .collect::<Vec<usize>>()
    .as_slice()
    .try_into()
    .unwrap();

  let g: Vec<Vec<(u32, usize)>> = {
    let mut g: Vec<Vec<(u32, usize)>> = vec![vec![]; args[0]];
    for _ in 0..args[1] {
      buf = "".to_string();
      io::stdin().read_line(&mut buf).unwrap();
      let e: [u32; 3] = buf
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .try_into()
        .unwrap();
      g[(e[0] - 1) as usize].push((e[2], (e[1] - 1) as usize));
      g[(e[1] - 1) as usize].push((e[2], (e[0] - 1) as usize));
    }
    g
  };
  drop(buf);

  let mut acc: u64 = 0;
  let mut dist: Vec<u32> = vec![i32::MAX as u32; args[0]];
  let mut done: Vec<bool> = vec![false; args[0]];
  let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

  dist[0] = 0;
  heap.push(Reverse((0, 0)));
  while let Some(Reverse(x)) = heap.pop() {
    if done[x.1] {
      continue;
    }
    acc += dist[x.1] as u64;
    done[x.1] = true;
    for e in g[x.1].iter() {
      if dist[e.1] > e.0 {
        dist[e.1] = e.0;
        heap.push(Reverse((e.0, e.1)));
      }
    }
  }

  println!("{}", acc)
}
