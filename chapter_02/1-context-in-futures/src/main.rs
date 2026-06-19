use std::time::Duration;
use tokio::time::timeout;

const WORK_TIME: u64 = 1000;
const TIMEOUT: u64 = 500;

async fn slow_task() -> &'static str {
  tokio::time::sleep(Duration::from_millis(WORK_TIME)).await;
  "Slow task completed."
}

#[tokio::main]
async fn main() {
  let result = timeout(Duration::from_millis(TIMEOUT), slow_task()).await;

  match result {
    Ok(value) => println!("{}", value),
    Err(_) => println!("Task timed out."),
  }
}
