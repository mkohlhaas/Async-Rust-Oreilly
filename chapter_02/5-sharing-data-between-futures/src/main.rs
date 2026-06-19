use core::task::Poll;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use tokio::task::JoinHandle;
use tokio::time::Duration;

#[derive(Debug)]
enum CounterType {
  Increment,
  Decrement,
}

struct SharedData {
  counter: i32,
}

impl SharedData {
  fn increment(&mut self) {
    self.counter += 1;
  }
  fn decrement(&mut self) {
    self.counter -= 1;
  }
}

struct CounterFuture {
  counter_type: CounterType,
  data_reference: Arc<Mutex<SharedData>>,
  count: u32,
}

impl Future for CounterFuture {
  type Output = u32;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    std::thread::sleep(Duration::from_secs(1));
    let mut guard = match self.data_reference.try_lock() {
      Ok(guard) => guard,
      Err(error) => {
        println!("error for {:?}: {}", self.counter_type, error);
        cx.waker().wake_by_ref();
        return Poll::Pending;
      }
    };
    let value = &mut *guard;
    match self.counter_type {
      CounterType::Increment => {
        value.increment();
        println!("after increment: {}", value.counter);
      }
      CounterType::Decrement => {
        value.decrement();
        println!("after decrement: {}", value.counter);
      }
    }
    std::mem::drop(guard);
    self.count += 1;
    if self.count < 3 {
      cx.waker().wake_by_ref();
      Poll::Pending
    } else {
      Poll::Ready(self.count)
    }
  }
}

#[allow(dead_code)]
async fn count(
  count: u32,
  data: Arc<tokio::sync::Mutex<SharedData>>,
  counter_type: CounterType,
) -> u32 {
  for _ in 0..count {
    let mut data = data.lock().await;
    match counter_type {
      CounterType::Increment => {
        data.increment();
        println!("after increment: {}", data.counter);
      }
      CounterType::Decrement => {
        data.decrement();
        println!("after decrement: {}", data.counter);
      }
    }
    std::mem::drop(data);
    std::thread::sleep(Duration::from_secs(1));
  }
  count
}

#[tokio::main]
async fn main() {
  let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));
  let counter1 = CounterFuture {
    counter_type: CounterType::Increment,
    data_reference: shared_data.clone(),
    count: 0,
  };

  let counter2 = CounterFuture {
    counter_type: CounterType::Decrement,
    data_reference: shared_data.clone(),
    count: 0,
  };

  let handle_one: JoinHandle<u32> = tokio::task::spawn(counter1);
  let handle_two: JoinHandle<u32> = tokio::task::spawn(counter2);

  let (_, _) = tokio::join!(handle_one, handle_two);

  println!("Program finished.")
}

