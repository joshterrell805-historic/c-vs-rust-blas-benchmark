extern crate rand;
extern crate time;
extern crate rblas;

use rand::random;
use time::precise_time_ns;
use rblas::*;

const EXAMPLES : usize = 100000;

fn main() {

  let mut times = [0u64 ; EXAMPLES];
  let mut dots = [0.0f64 ; EXAMPLES];

  for i in 0..EXAMPLES {
    let v = vec![
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    ];
    let w = vec![
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    ];

    let start = precise_time_ns();
    dots[i] = Dot::dot(&v, &w);
    let end = precise_time_ns();

    times[i] = end - start;
  }
  
  let mut sum = 0u64;

  for i in 0..EXAMPLES {
    sum += times[i];
  }

  let avg = (sum as usize) / EXAMPLES;

  println!("average dot product computation: {:?}ns", avg);
  println!("first dot: {:?}, first time: {:?}", dots[0], times[0]);
  println!("mid dot: {:?}, mid time: {:?}", dots[EXAMPLES/2],
      times[EXAMPLES/2]);
  println!("last dot: {:?}, last time: {:?}", dots[EXAMPLES-1],
      times[EXAMPLES-1]);
}
