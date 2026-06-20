use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
  // Pair of Mutex and Condvar wrapped up in an Arc.
  // Condvar serves as a notification/communication system between threads.
  let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
  let shared_data_clone = Arc::clone(&shared_data);

  let stop = Arc::new(AtomicBool::new(false));
  let stop_clone = Arc::clone(&stop);

  // receiver thread
  // consumes `shared_data_clone` and `stop_clone`
  let _ = thread::spawn(move || {
    let (mtx, cvar) = &*shared_data_clone;
    let mut received_value = mtx.lock().unwrap();
    while !stop_clone.load(Relaxed) {
      received_value = cvar.wait(received_value).unwrap(); // blocks till we got a notification
      println!("Received value: {}", *received_value); // will also be printed on STOP
    }
  });

  // consumes `shared_data` and `stop`
  let sender_thread = thread::spawn(move || {
    let (mtx, cvar) = &*shared_data;
    let values = [
      false, true, false, true, true, true, true, true, false, false, false, false, false,
    ];

    // changing condvar
    for i in 0..values.len() {
      let update_value = values[i as usize];
      println!("Sending {update_value}...");
      *mtx.lock().unwrap() = update_value;
      cvar.notify_one(); // wake up our receiver thread
      thread::sleep(Duration::from_millis(500));
    }

    // stop endless loop in receiver thread
    stop.store(true, Relaxed);
    println!("STOP has been sent");
    cvar.notify_one(); // notify our receiver thread
  });

  sender_thread.join().unwrap();
}
