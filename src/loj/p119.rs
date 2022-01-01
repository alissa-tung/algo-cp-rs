use std::{cmp::Reverse, collections::BinaryHeap, convert::TryInto, io};

pub fn _main() {
  let mut buf = String::new();

  io::stdin().read_line(&mut buf).unwrap();
  let args: [usize; 4] = buf
    .split_ascii_whitespace()
    .take(4)
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
      let edge: [u32; 3] = buf
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .try_into()
        .unwrap();
      g[(edge[0] - 1) as usize].push((edge[2], (edge[1] - 1) as usize));
      g[(edge[1] - 1) as usize].push((edge[2], (edge[0] - 1) as usize));
    }
    g
  };

  drop(buf);

  let mut dist: Vec<u32> = vec![i32::MAX as u32; args[0]];
  let mut done: Vec<bool> = vec![false; args[0]];
  let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

  dist[args[2] - 1] = 0;
  heap.push(Reverse((0, args[2] - 1)));

  while let Some(Reverse(x)) = heap.pop() {
    if done[x.1] {
      continue;
    }
    for edge in g[x.1].iter() {
      if dist[edge.1] > dist[x.1] + edge.0 {
        dist[edge.1] = dist[x.1] + edge.0;
        heap.push(Reverse((dist[edge.1], edge.1)));
      }
    }
    done[x.1] = true;
  }

  println!("{}", dist[args[3] - 1]);
}
