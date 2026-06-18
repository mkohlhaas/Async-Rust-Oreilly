use std::env;
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::{Duration, Instant};

// they are calling themselves (this binary!)
fn run_processes() {
  let mut process1 = Command::new(env::current_exe().unwrap())
    .arg("task")
    .arg("1")
    .spawn()
    .expect("Failed to start process #1");

  let mut process2 = Command::new(env::current_exe().unwrap())
    .arg("task")
    .arg("11")
    .spawn()
    .expect("Failed to start process #2");

  process1.wait().expect("Failed to wait for process #1");
  process2.wait().expect("Failed to wait for process #2");

  println!("Both processes have completed.");
}

fn task(start_task_number: usize) {
  for i in start_task_number..start_task_number + 10 {
    sleep(Duration::from_millis(500));
    println!("Task {i} completed in process {}.", std::process::id());
  }
  exit(0);
}

fn main() {
  let start = Instant::now();
  let args: Vec<String> = env::args().collect();

  if args.len() > 2 && args[1] == "task" {
    let start_task_number = args[2].parse::<usize>().unwrap();
    task(start_task_number);
  } else {
    run_processes();
  }

  if args.len() <= 1 {
    let elapsed = start.elapsed();
    println!("The whole program took {:?} seconds.", elapsed.as_secs());
  }
}
