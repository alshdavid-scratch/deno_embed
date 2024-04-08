const THREAD_ID = parseInt(new URL(import.meta.url).searchParams.get('i'), 10)

// console.log(THREAD_ID, globalThis?.Extensions?.hello_world() || 'No func')

globalThis?.Extensions?.register_callback((value) => console.log(`[${THREAD_ID}] ${value}`))
