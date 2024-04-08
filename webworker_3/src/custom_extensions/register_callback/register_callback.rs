use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use deno_core::*;
use once_cell::sync::Lazy;

pub static REGISTER_CALLBACK: Lazy<Mutex<(Sender<Sender<usize>>, Option<Receiver<Sender<usize>>>)>> = Lazy::new(|| {
  let (tx, rx) = channel();
  Mutex::new((tx, Some(rx)))
});


#[op2(reentrant)]
pub fn op_register_callback<'s>(
  scope: &'s mut v8::HandleScope,
  callback: v8::Local<v8::Function>,
) {
  let (tx, rx) = channel::<usize>();

  REGISTER_CALLBACK.lock().unwrap().0.send(tx).unwrap();

  while let Ok(msg) = rx.recv() {
    let recv = v8::undefined(scope);
    let value = serde_v8::to_v8(scope, msg).unwrap();
    callback.call(scope, recv.into(), &[value]);
  }
}

deno_core::extension!(
  custom_register_callback,
  ops = [op_register_callback],
  esm_entry_point = "ext:custom_register_callback/src/custom_extensions/register_callback/register_callback.js",
  esm = ["src/custom_extensions/register_callback/register_callback.js"]
);
