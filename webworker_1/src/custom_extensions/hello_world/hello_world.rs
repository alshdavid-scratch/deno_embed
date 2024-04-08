use deno_core::*;

#[op2(fast)]
pub fn op_hello_world() {
  println!("hello from rust");
}

deno_core::extension!(
  custom_hello_world,
  ops = [op_hello_world],
  esm_entry_point = "ext:custom_hello_world/src/custom_extensions/hello_world/hello_world.js",
  esm = ["src/custom_extensions/hello_world/hello_world.js"],
);
