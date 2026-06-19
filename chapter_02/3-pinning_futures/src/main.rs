use std::future::Future;
use std::pin::Pin;
use std::ptr; // manually manage memory through raw pointers
use std::task::{Context, Poll};

struct SelfReferential {
  data: String,
  self_pointer: *const String, // points to `data`
}

impl SelfReferential {
  fn new(data: String) -> SelfReferential {
    let mut sr = SelfReferential {
      data,
      self_pointer: ptr::null(),
    };
    sr.self_pointer = &sr.data as *const String;
    sr
  }

  fn print(&self) {
    unsafe {
      println!("{}", *self.self_pointer);
    }
  }
}

#[allow(dead_code)]
struct Counter {
  count: u32,
}

impl Future for Counter {
  type Output = u32;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.count += 1;

    if self.count < 3 {
      cx.waker().wake_by_ref();
      Poll::Pending
    } else {
      Poll::Ready(self.count)
    }
  }
}

fn main() {
  let first = SelfReferential::new("first".to_string());
  let moved = first;

  // The original `first` is no longer valid; this might invalidate pointers if pinning isn't used.
  moved.print();

  println!("Program finished.")
}

