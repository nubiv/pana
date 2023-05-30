<script lang="ts">
import { Container } from '@svelteuidev/core'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { writable, type Writable } from 'svelte/store'

export const output: Writable<string> = writable('Drop output here.')

type TPayload = {
  message: string
}

const unlisten = listen('incoming_response', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  const res = event.payload as TPayload
  console.log(res.message)
  output.update((pre) => `${pre}\n${res.message}`)
})
console.log('start listening...')

onMount(() => {
  console.log('on mounting...')
})

onDestroy(async () => {
  await unlisten.then(() => console.log('stop listening...'))
})
</script>

<Container
  override="{{
    bc: 'AliceBlue',
    whiteSpace: 'pre-wrap'
  }}">
  {$output}
</Container>
