use std::thread::sleep;
use std::time::{Duration, Instant};

fn task() {
  println!("Running task…");
  sleep(Duration::from_secs(1));
}

fn main() {
  let start = Instant::now();

  for _ in 0..4 {
    task();
  }

  println!(
    "The whole program took {:?} seconds.",
    start.elapsed().as_secs()
  );
}
