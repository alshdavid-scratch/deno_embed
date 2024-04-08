mod deno_embed;
mod deno_cli;
mod custom_extensions;

use deno_embed::deno_current_thread;
use custom_extensions::REGISTER_CALLBACK;

const THREADS: usize = 1;

fn main() {
  let rx = REGISTER_CALLBACK.lock().unwrap().1.take().unwrap();

  let h1 = std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "./javascript/index.js".to_string(),
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();
    });
  });

  let mut senders = vec![];
  let mut c = 0;

  while let Ok(sender) = rx.recv() {
    senders.push(sender);
    c += 1;
    if c == THREADS {
      break;
    }
  }

  for sender in senders.iter() {
    sender.send(42).unwrap();
  }

  for sender in senders.iter() {
    sender.send(42).unwrap();
  }

  drop(senders);
  h1.join().unwrap();
}
