<script lang="ts">
import { listen } from '@tauri-apps/api/event'
import Tabs from './lib/components/Tabs.svelte'
import { onDestroy } from 'svelte'
import { llmState } from './lib/store/llm-state'

type TPayload = {
  message: string
}

const unlisten = listen('system_message', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  const res = event.payload as TPayload
  console.log(res.message)
  alert(res.message)

  if (res.message === 'Llama activated...') {
    llmState.set(true)
  }
})

onDestroy(async () => {
  await unlisten.then(() => console.log('stop listening...'))
})
</script>

<main class="container">
  <h1>Lobot. &#129302;</h1>
  <Tabs />
</main>
