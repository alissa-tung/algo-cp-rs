use std::{
  cmp::Ordering::*,
  io,
  mem::{replace, swap},
};

#[allow(dead_code)]

pub fn main() {
  let mut buf = String::new();
  io::stdin().read_line(&mut buf).unwrap();
  let arg = buf.trim().parse::<u32>().unwrap();

  let mut ns = NS(None, Rand::new());

  for _ in 0..arg {
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let args: [i32; 2] = buf
      .split_ascii_whitespace()
      .map(|x| x.parse().unwrap())
      .collect::<Vec<i32>>()
      .try_into()
      .unwrap();
    match args[0] {
      1 => ns.insert(args[1]),
      2 => ns.delete(args[1]),
      3 => println!("{}", rank(&ns.0, args[1]) + 1),
      4 => println!("{}", k_th(&ns.0, args[1] as u32)),
      5 => println!("{}", pre(&ns.0, args[1]).unwrap()),
      6 => println!("{}", suc(&ns.0, args[1]).unwrap()),
      _ => panic!(),
    }
  }
}

// -----------------------------------------------------------------------------

fn pre(xs: &Option<Box<N>>, x: i32) -> Option<i32> {
  if xs.is_none() {
    None
  } else {
    match xs.as_ref().unwrap().kvn.1.cmp(&x) {
      Less => {
        let ret = pre(&xs.as_ref().unwrap().r, x);
        if ret.is_some() {
          ret
        } else if xs.as_ref().unwrap().kvn.2 != 0 {
          Some(xs.as_ref().unwrap().kvn.1)
        } else {
          pre(&xs.as_ref().unwrap().l, x)
        }
      }
      _ => pre(&xs.as_ref().unwrap().l, x),
    }
  }
}

fn suc(xs: &Option<Box<N>>, x: i32) -> Option<i32> {
  if xs.is_none() {
    None
  } else {
    match xs.as_ref().unwrap().kvn.1.cmp(&x) {
      Greater => {
        let ret = suc(&xs.as_ref().unwrap().l, x);
        if ret.is_some() {
          ret
        } else if xs.as_ref().unwrap().kvn.2 != 0 {
          Some(xs.as_ref().unwrap().kvn.1)
        } else {
          suc(&xs.as_ref().unwrap().r, x)
        }
      }
      _ => suc(&xs.as_ref().unwrap().r, x),
    }
  }
}

fn rank(xs: &Option<Box<N>>, x: i32) -> u32 {
  if xs.is_none() {
    0
  } else {
    match xs.as_ref().unwrap().kvn.1.cmp(&x) {
      Equal => {
        if xs.as_ref().unwrap().l.is_some() {
          xs.as_ref().unwrap().l.as_ref().unwrap().s
        } else {
          0
        }
      }
      Less => {
        xs.as_ref().unwrap().kvn.2
          + ({
            if xs.as_ref().unwrap().l.is_some() {
              xs.as_ref().unwrap().l.as_ref().unwrap().s
            } else {
              0
            }
          })
          + rank(&xs.as_ref().unwrap().r, x)
      }
      Greater => rank(&xs.as_ref().unwrap().l, x),
    }
  }
}

fn k_th(xs: &Option<Box<N>>, k: u32) -> i32 {
  if xs.is_none() || k > xs.as_ref().unwrap().s {
    panic!()
  } else {
    let s = if xs.as_ref().unwrap().l.is_none() {
      0
    } else {
      xs.as_ref().unwrap().l.as_ref().unwrap().s
    };
    if s < k && k <= s + xs.as_ref().unwrap().kvn.2 {
      xs.as_ref().unwrap().kvn.1
    } else if k <= s {
      k_th(&xs.as_ref().unwrap().l, k)
    } else {
      k_th(&xs.as_ref().unwrap().r, k - s - xs.as_ref().unwrap().kvn.2)
    }
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct N {
  l: Option<Box<N>>,
  r: Option<Box<N>>,
  kvn: (u32, i32, u32),
  s: u32,
}

impl N {
  fn new(rand: &mut Rand, x: i32) -> Box<N> {
    Box::new(N {
      l: None,
      r: None,
      kvn: (rand.next_as_u32(), x, 1),
      s: 0,
    })
  }
}

fn size(x: &mut Box<N>) {
  x.as_mut().s = x.as_ref().kvn.2;
  if x.as_ref().l.is_some() {
    x.as_mut().s += x.l.as_ref().unwrap().s;
  }
  if x.as_ref().r.is_some() {
    x.as_mut().s += x.r.as_ref().unwrap().s;
  }
}

fn rotate(n_0: &mut Box<N>, b: bool) {
  if b {
    let n_2 = n_0.r.as_mut().unwrap();
    swap(&mut n_0.kvn, &mut n_2.kvn);
    let n_1 = n_0.l.take();
    let n_3 = n_2.l.take();
    let n_4 = n_2.r.take();

    let mut l = replace(&mut n_0.r, n_4);
    {
      let l = l.as_mut().unwrap();
      l.l = n_1;
      l.r = n_3;
    }
    n_0.l = l;
  } else {
    let n_1 = n_0.l.as_mut().unwrap();
    swap(&mut n_0.kvn, &mut n_1.kvn);
    let n_2 = n_0.r.take();
    let n_3 = n_1.l.take();
    let n_4 = n_1.r.take();

    let mut r = replace(&mut n_0.l, n_3);
    {
      let r = r.as_mut().unwrap();
      r.l = n_4;
      r.r = n_2;
    }
    n_0.r = r;
  }
}

#[derive(Debug)]
struct NS(Option<Box<N>>, Rand);

impl NS {
  fn insert(&mut self, x: i32) {
    if self.0.is_none() {
      self.0 = Some(N::new(&mut self.1, x));
      size(self.0.as_mut().unwrap());
    } else {
      match self.0.as_ref().unwrap().kvn.1.cmp(&x) {
        Equal => {
          self.0.as_mut().unwrap().kvn.2 += 1;
          size(self.0.as_mut().unwrap());
        }
        Less => {
          let ys = self.0.as_mut().unwrap().r.take();
          let xs = replace(&mut self.0, ys);
          self.insert(x);
          let ys = replace(&mut self.0, xs);
          self.0.as_mut().unwrap().r = ys;
          if self.0.as_ref().unwrap().r.as_ref().unwrap().kvn.0 > self.0.as_ref().unwrap().kvn.0 {
            rotate(self.0.as_mut().unwrap(), true);
          }
          if self.0.as_ref().unwrap().l.is_some() {
            size(self.0.as_mut().unwrap().l.as_mut().unwrap())
          }
          size(self.0.as_mut().unwrap());
        }
        Greater => {
          let ys = self.0.as_mut().unwrap().l.take();
          let xs = replace(&mut self.0, ys);
          self.insert(x);
          let ys = replace(&mut self.0, xs);
          self.0.as_mut().unwrap().l = ys;
          if self.0.as_ref().unwrap().l.as_ref().unwrap().kvn.0 > self.0.as_ref().unwrap().kvn.0 {
            rotate(self.0.as_mut().unwrap(), false);
          }
          if self.0.as_ref().unwrap().r.is_some() {
            size(self.0.as_mut().unwrap().r.as_mut().unwrap())
          }
          size(self.0.as_mut().unwrap());
        }
      }
    }
  }

  fn delete(&mut self, x: i32) {
    // #TODO: As for traversal operations, there is no need to impl them as methods.
    match self.0.as_ref().unwrap().kvn.1.cmp(&x) {
      Equal => {
        self.0.as_mut().unwrap().kvn.2 -= 1;
        size(self.0.as_mut().unwrap());
      }
      Less => {
        let ys = self.0.as_mut().unwrap().r.take();
        let xs = replace(&mut self.0, ys);
        self.delete(x);
        let ys = replace(&mut self.0, xs);
        self.0.as_mut().unwrap().r = ys;
        size(self.0.as_mut().unwrap().r.as_mut().unwrap());
        size(self.0.as_mut().unwrap());
      }
      Greater => {
        let ys = self.0.as_mut().unwrap().l.take();
        let xs = replace(&mut self.0, ys);
        self.delete(x);
        let ys = replace(&mut self.0, xs);
        self.0.as_mut().unwrap().l = ys;
        size(self.0.as_mut().unwrap().l.as_mut().unwrap());
        size(self.0.as_mut().unwrap());
      }
    }
  }
}

// impl NS {
//     fn valid(&self) {
//         if self.0.is_none() {
//         } else {
//             let xs = self.0.as_ref().unwrap();
//             assert!(xs.l.is_none() || xs.l.as_ref().unwrap().kvn.0 < xs.kvn.0);
//             assert!(xs.r.is_none() || xs.r.as_ref().unwrap().kvn.0 < xs.kvn.0);
//             assert!(xs.l.is_none() || xs.l.as_ref().unwrap().kvn.1 < xs.kvn.1);
//             assert!(xs.r.is_none() || xs.r.as_ref().unwrap().kvn.1 > xs.kvn.1);
//             assert!({
//                 let l_s = if xs.l.is_none() {
//                     0
//                 } else {
//                     xs.l.as_ref().unwrap().s
//                 };
//                 let r_s = if xs.r.is_none() {
//                     0
//                 } else {
//                     xs.r.as_ref().unwrap().s
//                 };
//                 l_s + r_s + xs.kvn.2 == xs.s
//             });
//         }
//     }

//     fn _println(&self) {
//         println!("{:#?}", self)
//     }
// }

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct Rand {
  s: [u64; 2],
}

impl Iterator for Rand {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let mut s1 = self.s[0];
    let s0 = self.s[1];
    let ret = s0.wrapping_add(s1);
    self.s[0] = s0;
    s1 ^= s1 << 23;
    self.s[1] = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5);
    Some(ret)
  }
}

impl Rand {
  fn new() -> Rand {
    Rand {
      s: [
        std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap()
          .as_millis() as u64,
        std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap()
          .as_millis() as u64,
      ],
    }
  }
  fn next_as_u32(&mut self) -> u32 {
    self.next().unwrap() as u32
  }
}
