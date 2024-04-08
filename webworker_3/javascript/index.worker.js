const THREAD_ID = parseInt(new URL(import.meta.url).searchParams.get('i'), 10) || 0
console.log(`[${THREAD_ID}] initializing`)

globalThis.Extensions.register_callback((value) => {
  console.log(`[${THREAD_ID}] ${value}`)
})
