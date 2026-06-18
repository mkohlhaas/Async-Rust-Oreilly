use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let url = "https://jsonplaceholder.typicode.com/posts/1";

  // Synchronous
  {
    let start_time = Instant::now();

    let _ = reqwest::get(url).await?; // await blocks!!!
    println!("finished #1");
    let _ = reqwest::get(url).await?;
    println!("finished #2");
    let _ = reqwest::get(url).await?;
    println!("finished #3");
    let _ = reqwest::get(url).await?;
    println!("finished #4");

    println!();
    let elapsed_time = start_time.elapsed();
    println!("Request took {} ms", elapsed_time.as_millis());
  }

  // Asynchronous
  {
    let start_time = Instant::now();

    let (_, _, _, _) = tokio::join!(
      reqwest::get(url),
      reqwest::get(url),
      reqwest::get(url),
      reqwest::get(url),
    );

    let elapsed_time = start_time.elapsed();
    println!("Request took {} ms", elapsed_time.as_millis());
  }

  Ok(())
}
