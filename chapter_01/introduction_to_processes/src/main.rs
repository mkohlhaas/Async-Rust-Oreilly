use std::thread::sleep;
use std::time::{Duration, Instant};

fn task(n: i32) {
  println!("Running task {n}…");
  sleep(Duration::from_millis(500));
}

fn main() {
  let start = Instant::now();

  for i in 1..7 {
    task(i);
  }

  println!(
    "The whole program took {:?} seconds.",
    start.elapsed().as_secs()
  );
}
