use futures_util::future::join_all;
use std::fs::{File, OpenOptions};
use std::future::Future;
use std::io::prelude::*;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

type AsyncFileHandle = Arc<Mutex<File>>;
type JoinHandleBool = JoinHandle<Result<bool, String>>;

fn get_handle(file_path: &dyn ToString) -> AsyncFileHandle {
  match OpenOptions::new().append(true).open(file_path.to_string()) {
    Ok(file) => Arc::new(Mutex::new(file)), // open file
    Err(_) => Arc::new(Mutex::new(File::create(file_path.to_string()).unwrap())), // create file
  }
}

struct AsyncWriteFuture {
  pub handle: AsyncFileHandle,
  pub entry: String,
}

impl Future for AsyncWriteFuture {
  type Output = Result<bool, String>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut guard = match self.handle.try_lock() {
      Ok(guard) => guard,
      Err(err) => {
        println!("error for {} : {}", self.entry, err);
        cx.waker().wake_by_ref();
        return Poll::Pending;
      }
    };

    let lined_entry = format!("{}\n", self.entry);
    match guard.write_all(lined_entry.as_bytes()) {
      Ok(_) => println!("written for: {}", self.entry),
      Err(err) => println!("Some weird error: {}", err),
    };
    Poll::Ready(Ok(true))
  }
}

// NOTE: this is not an `async` fn
fn write_log(file_handle: AsyncFileHandle, line: String) -> JoinHandleBool {
  let my_future = AsyncWriteFuture {
    handle: file_handle,
    entry: line,
  };
  tokio::task::spawn(my_future)
}

#[tokio::main]
async fn main() {
  let login_file = get_handle(&"login.txt");
  let logout_file = get_handle(&"logout.txt");

  let names = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
  ];
  let mut handles = Vec::new();

  for name in names {
    let file1 = login_file.clone();
    let task1 = write_log(file1, name.to_string() + " login");
    handles.push(task1);

    let file2 = logout_file.clone();
    let task2 = write_log(file2, name.to_string() + " logout");
    handles.push(task2);
  }
  println!("\nNumber of tasks: {}\n", handles.len());

  let _ = join_all(handles).await;
}
