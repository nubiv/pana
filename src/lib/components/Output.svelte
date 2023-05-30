<script lang="ts">
import { Container } from '@svelteuidev/core'
import { listen } from '@tauri-apps/api/event'
import { onDestroy } from 'svelte'
import { writable, type Writable } from 'svelte/store'

export const output: Writable<string> = writable('')

type TPayload = {
  message: string
}

const unlisten = listen('incoming_response', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  const res = event.payload as TPayload
  console.log('start listening...')
  output.update((pre) => `${pre}\nLlama: ${res.message}`)
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
