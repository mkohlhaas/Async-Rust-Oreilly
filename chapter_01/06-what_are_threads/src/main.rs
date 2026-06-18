use std::thread;
use std::time::Instant;

const N: u64 = 50000;

fn fibonacci(n: u64) -> u64 {
  if n == 0 || n == 1 {
    return n;
  }
  fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
  let start = Instant::now();
  let mut handles = Vec::new();

  for _ in 0..4 {
    let handle = thread::spawn(|| fibonacci(N));
    handles.push(handle);
  }

  // ⚠️ No joins!!!

  println!("fibonacci of {N} took {:?} seconds.", start.elapsed());
}
