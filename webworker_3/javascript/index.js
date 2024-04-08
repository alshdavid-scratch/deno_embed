// const workers = []

// for (let i = 0; i < (Deno.args[0] || 3); i++) {
//   workers.push(new Worker(import.meta.resolve(`./index.worker.js?i=${i+1}`), { type: "module" }));
// }

setTimeout(() => {
  Extensions.register_callback((value) => console.log(`[${0}] ${value}`))
})

console.log('b')

// for (const worker of workers) {
//   worker.terminate()
// }
