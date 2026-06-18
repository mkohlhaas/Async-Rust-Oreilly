use std::io::{self, BufRead};
use std::process;

fn main() {
  println!("process ID: {}", process::id());

  let mut lines = io::stdin().lock().lines();

  while let Some(Ok(line)) = lines.next() {
    println!("Received: {}", line);
  }

  println!("Bye, bye!");
}
