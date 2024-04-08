mod deno_embed;
mod deno_cli;
mod custom_extensions;

use deno_embed::deno_current_thread;
use custom_extensions::custom_hello_world;


fn main() {
  
  std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "./javascript/index.js"
          .to_string(),
        extensions: vec![
          custom_hello_world::init_ops_and_esm()
        ],
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();
    });
  }).join().unwrap();
}