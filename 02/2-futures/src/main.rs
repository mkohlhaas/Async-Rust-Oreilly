use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct Counter {
  count: u32,
}

impl Future for Counter {
  type Output = u32;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.count += 1;

    println!("Polling with result: {}", self.count);

    std::thread::sleep(Duration::from_millis(500));

    if self.count < 3 {
      cx.waker().wake_by_ref(); // wake up the task
      Poll::Pending
    } else {
      Poll::Ready(self.count)
    }
  }
}

#[tokio::main]
async fn main() {
  let counter1 = Counter { count: 0 };
  let counter2 = Counter { count: 0 };

  let handle1 = tokio::spawn(counter1);
  let handle2 = tokio::spawn(counter2);

  let (_, _) = tokio::join!(handle1, handle2);

  println!("Program finished!")
}
