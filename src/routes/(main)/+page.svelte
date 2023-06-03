<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { llmState } from '$lib/store/llm-state'
import Output from '$lib/components/Output.svelte'
import Input from '$lib/components/Input.svelte'

type TPayload = {
  message: string
}

let unlisten: any

invoke('update_llm_models')
  .then((res) => {
    console.log(res)
  })
  .catch((e) => console.log(e))

onMount(async () => {
  unlisten = listen('system_message', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event)
    const res = event.payload as TPayload

    if (res.message === 'Llama activated...') {
      llmState.set(true)
      alert(res.message)
    }
  })
})

onDestroy(async () => {
  await unlisten.then(() => console.log('stop listening...'))
})
</script>

<main class="container">
  <h1 class=" text-center text-xl p-3">Lobot. &#129302;</h1>
  <Output />
  <Input />
</main>
