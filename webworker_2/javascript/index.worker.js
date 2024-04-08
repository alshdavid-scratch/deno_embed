const THREAD_ID = new URL(import.meta.url).searchParams.get('i') || 1

console.log(THREAD_ID, globalThis?.Extensions?.hello_world() || 'No func')
