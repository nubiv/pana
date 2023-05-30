import { writable, type Writable } from 'svelte/store'

export const llmState: Writable<boolean> = writable(false)
