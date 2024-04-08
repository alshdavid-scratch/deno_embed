// Works from main threads
console.log(Extensions.hello_world())

const workers = []

// Spawn threads
for (let i = 0; i < (Deno.args[0] || 3); i++) {
  workers.push(new Worker(import.meta.resolve(`./index.worker.js?i=${i}`), { type: "module" }));
}

// Deno will hang if workers are not manually terminated
for (const worker of workers) {
  worker.terminate()
}
