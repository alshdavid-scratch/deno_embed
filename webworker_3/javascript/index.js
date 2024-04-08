const THREADS = Deno.args[0] ? parseInt(Deno.args[0], 10): 1
const THREAD_ID = 0;
console.log(`[${THREAD_ID}] initializing`)

const workers = []

// the main thread is 1 thread
for (let i = 1; i < THREADS; i++) {
  workers.push(new Worker(import.meta.resolve(`./index.worker.js?i=${i+1}`), { type: "module" }));
}

globalThis.Extensions.register_callback((value) => {
  console.log(`[${THREAD_ID}] ${value}`)
})

for (const worker of workers) {
  worker.terminate()
}
