import { writable, type Writable } from 'svelte/store'

export const output: Writable<string> = writable('')
