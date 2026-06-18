use std::thread;
use std::thread::available_parallelism;
use std::time::Instant;

const N: u64 = 41;

fn fibonacci(n: u64) -> u64 {
  if n == 0 || n == 1 {
    return n;
  }
  fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
  let num_cpu = available_parallelism().unwrap().get();

  let start = Instant::now();
  let mut handles = Vec::new();

  for _ in 0..num_cpu {
    let handle = thread::spawn(|| fibonacci(N));
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!(
    "{num_cpu} threads of fibonacci of {N} took {:?} seconds.",
    start.elapsed()
  );
}
