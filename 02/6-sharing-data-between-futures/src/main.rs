use core::task::Poll;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::thread::sleep;
use tokio::task::JoinHandle;
use tokio::time::Duration;

const MAX_COUNT: u32 = 10;
const SLEEP_TIME: u64 = 200;

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

struct Counter {
  counter_type: CounterType,
  data_reference: Arc<Mutex<SharedData>>,
  count: u32,
}

impl Future for Counter {
  type Output = u32;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    sleep(Duration::from_millis(SLEEP_TIME));

    let mut guard = match self.data_reference.try_lock() {
      Ok(guard) => guard,
      Err(err) => {
        println!("error for {:?}: {}", self.counter_type, err);
        cx.waker().wake_by_ref();
        return Poll::Pending;
      }
    };

    let shared_data = &mut *guard;
    match self.counter_type {
      CounterType::Increment => {
        shared_data.increment();
        println!("after increment: {}", shared_data.counter);
      }
      CounterType::Decrement => {
        shared_data.decrement();
        println!("after decrement: {}", shared_data.counter);
      }
    }

    std::mem::drop(guard);

    self.count += 1;
    if self.count < MAX_COUNT {
      cx.waker().wake_by_ref();
      Poll::Pending
    } else {
      Poll::Ready(self.count)
    }
  }
}

async fn count(
  data: Arc<tokio::sync::Mutex<SharedData>>, // NOTE: using a tokio mutex
  counter_type: CounterType,
) {
  for _ in 0..MAX_COUNT {
    let mut shared_data = data.lock().await;
    match counter_type {
      CounterType::Increment => {
        shared_data.increment();
        println!("after increment: {}", shared_data.counter);
      }
      CounterType::Decrement => {
        shared_data.decrement();
        println!("after decrement: {}", shared_data.counter);
      }
    }
    std::mem::drop(shared_data);
    sleep(Duration::from_millis(SLEEP_TIME));
  }
}

#[tokio::main]
async fn main() {
  // Standard Mutex
  {
    let shared_data = Arc::new(Mutex::new(SharedData {
      counter: Default::default(),
    }));

    let counter1 = Counter {
      counter_type: CounterType::Increment,
      data_reference: shared_data.clone(),
      count: Default::default(),
    };

    let counter2 = Counter {
      counter_type: CounterType::Decrement,
      data_reference: shared_data.clone(),
      count: Default::default(),
    };

    let handle_one: JoinHandle<u32> = tokio::task::spawn(counter1);
    let handle_two: JoinHandle<u32> = tokio::task::spawn(counter2);

    let (res1, res2) = tokio::join!(handle_one, handle_two);

    println!("{res1:?}");
    println!("{res2:?}");
  }

  println!("\n// Using mutexes from Tokio! //\n");

  // Tokio Mutex
  {
    let shared_data = Arc::new(tokio::sync::Mutex::new(SharedData { counter: 0 }));
    let shared_two = shared_data.clone();

    let handle_one: JoinHandle<_> =
      tokio::task::spawn(async move { count(shared_data, CounterType::Increment).await });
    let handle_two: JoinHandle<_> =
      tokio::task::spawn(async move { count(shared_two, CounterType::Decrement).await });

    let _ = tokio::join!(handle_one, handle_two);
  }

  println!("Program finished.")
}
