<style>
p {
  white-space: pre-line;
}
</style>

<script lang="ts">
import { listen } from '@tauri-apps/api/event'
import { onDestroy } from 'svelte'
import { output } from '$store/output'
import { AspectRatio } from './ui/aspect-ratio'

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

<AspectRatio
  ratio="{16 / 9}"
  class="bg-muted flex flex-col-reverse overflow-scroll ">
  <p>
    {$output}
  </p>
</AspectRatio>
