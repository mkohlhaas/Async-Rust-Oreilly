use std::thread;

fn fibonacci(n: u64) -> u64 {
  if n == 0 || n == 1 {
    return n;
  }
  fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
  let mut handles = Vec::new();

  for i in 0..8 {
    let handle = thread::spawn(move || {
      let result = fibonacci(20); // fibonacci(4000)
      println!("Thread {}: {}", i, result);
      result
    });
    handles.push(handle);
  }

  // wait for every thread to be finished (-> blocking)
  for handle in handles {
    handle.join().unwrap();
  }
}
