use async_task::{Runnable, Task};
use futures_lite::future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

const MAX_COUNT: u32 = 3;
const SLEEP_TIME: u64 = 200;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum FutureType {
  High,
  Low,
}

trait FutureOrderLabel: Future {
  fn get_order(&self) -> FutureType;
}

fn spawn_task<F, T>(future: F) -> Task<T>
where
  F: Future<Output = T> + Send + 'static + FutureOrderLabel,
  T: Send + 'static,
{
  static HIGH_QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
    let (tx, rx) = flume::unbounded::<Runnable>();
    for _ in 0..2 {
      let receiver = rx.clone();
      thread::spawn(move || {
        while let Ok(runnable) = receiver.recv() {
          let _ = catch_unwind(|| runnable.run());
        }
      });
    }
    tx
  });

  static LOW_QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
    let (tx, rx) = flume::unbounded::<Runnable>();
    for _ in 0..1 {
      let receiver = rx.clone();
      thread::spawn(move || {
        while let Ok(runnable) = receiver.recv() {
          let _ = catch_unwind(|| runnable.run());
        }
      });
    }
    tx
  });

  let schedule_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
  let schedule_low = |runnable| LOW_QUEUE.send(runnable).unwrap();

  let schedule = match future.get_order() {
    FutureType::High => schedule_high,
    FutureType::Low => schedule_low,
  };

  let (runnable, task) = async_task::spawn(future, schedule);
  runnable.schedule();
  task
}

struct CounterFuture {
  count: u32,
  order: FutureType,
}

impl FutureOrderLabel for CounterFuture {
  fn get_order(&self) -> FutureType {
    self.order
  }
}
impl Future for CounterFuture {
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

#[allow(dead_code)]
async fn async_fn() {
  std::thread::sleep(Duration::from_millis(SLEEP_TIME));
  println!("async fn");
}

fn main() {
  let one = CounterFuture {
    count: 0,
    order: FutureType::High,
  };
  let t_one = spawn_task(one);
  future::block_on(t_one);

  // let one = CounterFuture { count: 0 };
  // let two = CounterFuture { count: 0 };
  // let t_one = spawn_task(one);
  // let t_two = spawn_task(two);
  // let t_three = spawn_task(async {
  //     async_fn().await;
  //     async_fn().await;
  //     async_fn().await;
  //     async_fn().await;
  // });
  // std::thread::sleep(Duration::from_secs(5));
  // println!("before the block");
  // future::block_on(t_one);
  // future::block_on(t_two);
  // future::block_on(t_three);
}
