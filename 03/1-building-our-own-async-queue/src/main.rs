use async_task::{Runnable, Task};
use futures_lite::future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

const SLEEP_TIME: u64 = 200;
const MAX_COUNT: u32 = 3;

fn spawn_task<F, T>(future: F) -> Task<T>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
    let (tx, rx) = flume::unbounded::<Runnable>();
    thread::spawn(move || {
      while let Ok(runnable) = rx.recv() {
        println!("Runnable accepted.");
        let _ = catch_unwind(|| runnable.run());
      }
    });
    tx
  });

  let schedule = |runnable| QUEUE.send(runnable).unwrap();
  let (runnable, task) = async_task::spawn(future, schedule);
  runnable.schedule();
  println!("Queue length: {:?}", QUEUE.len());
  task
}

struct Counter {
  count: u32,
}

impl Future for Counter {
  type Output = u32;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.count += 1;
    println!("Polling with result: {}", self.count);

    std::thread::sleep(Duration::from_millis(SLEEP_TIME));

    if self.count < MAX_COUNT {
      cx.waker().wake_by_ref();
      Poll::Pending
    } else {
      Poll::Ready(self.count)
    }
  }
}

async fn async_fn() {
  println!("async fn");
  std::thread::sleep(Duration::from_millis(SLEEP_TIME));
}

fn main() {
  let f1 = Counter { count: 0 };
  let f2 = Counter { count: 0 };

  let t1 = spawn_task(f1);
  let t2 = spawn_task(f2);
  let t3 = spawn_task(async {
    async_fn().await;
    async_fn().await;
    async_fn().await;
    async_fn().await;
  });

  std::thread::sleep(Duration::from_millis(SLEEP_TIME));

  println!("Before blocking.");

  future::block_on(t1);
  future::block_on(t2);
  future::block_on(t3);
}
