mod deno_embed;
mod deno_cli;
mod custom_extensions;
mod worker_farm;


use deno_embed::deno_current_thread;
use custom_extensions::REGISTER_CALLBACK;
use worker_farm::WorkerFarm;

const THREADS: usize = 5;

fn main() {
  let rx = REGISTER_CALLBACK.lock().unwrap().1.take().unwrap();

  let h1 = std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "./javascript/index.js".to_string(),
        args: vec![format!("{}", THREADS)],
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();
    });
  });

  let worker_farm = WorkerFarm::<usize>::from_rx(rx, THREADS);
  
  worker_farm.send_all(42);

  h1.join().unwrap();
}
