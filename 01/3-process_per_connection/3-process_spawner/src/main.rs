use std::io::Result;
use tokio::process::Command;

const NUM_PROCESSES: u8 = 4;
const CMD: &str = "../connection_bin";

#[tokio::main]
async fn main() -> Result<()> {
  let mut handles = Vec::new();

  for _ in 0..NUM_PROCESSES {
    let handle = tokio::spawn(async {
      let output = Command::new(CMD).output().await;

      match output {
        Ok(output) => {
          println!(
            "Process completed with output: {}",
            String::from_utf8_lossy(&output.stdout)
          );
          Ok(output.status.code().unwrap_or(-1))
        }
        Err(e) => {
          eprintln!("Failed to run process: {}", e);
          Err(e)
        }
      }
    });
    handles.push(handle);
  }

  let mut results = Vec::with_capacity(handles.len());
  for handle in handles {
    results.push(handle.await.unwrap());
  }

  results
    .into_iter()
    .enumerate()
    .for_each(|(idx, res)| match res {
      Ok(exit_code) => println!("Process {} exited with code {}", idx + 1, exit_code),
      Err(e) => eprintln!("Process {} failed: {}", idx + 1, e),
    });

  Ok(())
}
