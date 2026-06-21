use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

type State = Arc<Mutex<MyFutureState>>;

struct MyFuture {
  state: State,
}

struct MyFutureState {
  data: Option<Vec<u8>>,
  waker: Option<Waker>,
}

impl MyFuture {
  fn new() -> (Self, State) {
    let state = Arc::new(Mutex::new(MyFutureState {
      data: None,
      waker: None,
    }));
    (
      MyFuture {
        state: state.clone(),
      },
      state,
    )
  }
}

impl Future for MyFuture {
  type Output = String;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("Polling the future.");
    let mut state = self.state.lock().unwrap();

    if state.data.is_some() {
      let data = state.data.take().unwrap();
      Poll::Ready(String::from_utf8(data).unwrap())
    } else {
      state.waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }
}

#[tokio::main]
async fn main() {
  let (my_future, state) = MyFuture::new();
  let (sender, mut receiver) = mpsc::channel::<()>(1);
  let task_handle1 = task::spawn(my_future);

  sleep(tokio::time::Duration::from_secs(1)).await;

  println!("Spawning trigger task...");
  let _task_handle2 = task::spawn(async move {
    receiver.recv().await;
    let mut state = state.lock().unwrap();
    state.data = Some(b"Hello from the outside.".to_vec());

    loop {
      if let Some(waker) = state.waker.take() {
        waker.wake();
        break;
      }
    }
  });

  sender.send(()).await.unwrap();

  let outome = task_handle1.await.unwrap();
  println!("Task completed with outcome: {}", outome);

  // task_handle2.await.unwrap();

  println!("Program finished.")
}
