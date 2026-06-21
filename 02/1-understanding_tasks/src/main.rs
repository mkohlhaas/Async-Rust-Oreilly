use std::time::Duration;
use std::time::Instant;

async fn prep_coffee_mug() {
  tokio::time::sleep(Duration::from_millis(500)).await; // asynchronous version of std::thread::sleep(…)
  println!("Pouring milk... ");
  std::thread::sleep(Duration::from_millis(500)); // std::thread::sleep(…) is blocking(!)
  println!("...milk poured.");

  println!("Putting instant coffee... ");
  std::thread::sleep(Duration::from_millis(500));
  println!("...instant coffee put.");
}

async fn make_coffee() {
  println!("Boiling kettle... ");
  tokio::time::sleep(Duration::from_millis(500)).await;
  println!("...kettle boiled.");

  println!("Pouring boiled water...");
  std::thread::sleep(Duration::from_millis(500));
  println!("...boiled water poured.");
}

async fn make_toast() {
  println!("Putting bread in toaster... ");
  tokio::time::sleep(Duration::from_millis(500)).await;
  println!("...bread toasted.");

  println!("Buttering toasted bread... ");
  std::thread::sleep(Duration::from_millis(500));
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
    let person1 = tokio::task::spawn(async {
      let coffee_mug_step = prep_coffee_mug();
      let coffee_step = make_coffee();
      let toast_step = make_toast();
      tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });
    person1.await.unwrap();

    println!("It took: {} seconds", start_time.elapsed().as_secs());
  }
}
