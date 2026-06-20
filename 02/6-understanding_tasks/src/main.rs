use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

async fn prep_coffee_mug() {
  sleep(Duration::from_millis(500)).await;
  println!("Pouring milk... ");
  thread::sleep(Duration::from_millis(500));
  println!("...milk poured.");

  println!("Putting instant coffee... ");
  thread::sleep(Duration::from_millis(500));
  println!("...instant coffee put.");
}

async fn make_coffee() {
  println!("Boiling kettle... ");
  sleep(Duration::from_millis(500)).await;
  println!("...kettle boiled.");

  println!("Pouring boiled water...");
  thread::sleep(Duration::from_millis(500));
  println!("...boiled water poured.");
}

async fn make_toast() {
  println!("Putting bread in toaster... ");
  sleep(Duration::from_millis(500)).await;
  println!("...bread toasted.");

  println!("Buttering toasted bread... ");
  thread::sleep(Duration::from_millis(500));
  println!("...toasted bread buttered.");
}

#[tokio::main]
async fn main() {
  // Asynchronous
  {
    let start_time = Instant::now();

    // Futures
    let coffee_mug_step = prep_coffee_mug();
    let coffee_step = make_coffee();
    let toast_step = make_toast();

    tokio::join!(coffee_mug_step, coffee_step, toast_step);

    println!("It took: {} seconds", start_time.elapsed().as_secs());
  }

  println!();

  // Synchronous
  {
    let start_time = Instant::now();
    let person1 = tokio::task::spawn(async {
      prep_coffee_mug().await;
      make_coffee().await;
      make_toast().await;
    });
    person1.await.unwrap();

    println!("It took: {} seconds", start_time.elapsed().as_secs());
  }

  println!();

  // Asynchronous
  {
    let start_time = Instant::now();
    let person2 = tokio::task::spawn(async {
      let coffee_mug_step = prep_coffee_mug();
      let coffee_step = make_coffee();
      let toast_step = make_toast();
      tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });
    person2.await.unwrap();

    println!("It took: {} seconds", start_time.elapsed().as_secs());
  }
}
