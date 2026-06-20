use futures_util::future::join_all;
use std::fs::{File, OpenOptions};
use std::future::Future;
use std::io::prelude::*;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

type AsyncFileHandle = Arc<Mutex<File>>;
type FileJoinHandle = JoinHandle<Result<bool, String>>;

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
      Ok(grd) => grd,
      Err(err) => {
        println!("error for {} : {}", self.entry, err);
        cx.waker().wake_by_ref();
        return Poll::Pending;
      }
    };

    let lined_entry = format!("{}\n", self.entry);
    match guard.write_all(lined_entry.as_bytes()) {
      Ok(_) => println!("written for: {}", self.entry),
      Err(e) => println!("{}", e),
    };
    Poll::Ready(Ok(true))
  }
}

fn write_log(file_handle: AsyncFileHandle, line: String) -> FileJoinHandle {
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

  let names = ["one", "two", "three", "four", "five", "six"];
  let mut handles = Vec::new();

  for name in names {
    let file1 = login_file.clone();
    let file2 = logout_file.clone();

    let task1 = write_log(file1, name.to_string());
    let task2 = write_log(file2, name.to_string());

    handles.push(task1);
    handles.push(task2);
  }

  let _ = join_all(handles).await;
}
