<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { LLMState } from '$lib/store/llm'
import Output from '$lib/components/Output.svelte'
import Input from '$lib/components/Input.svelte'

type TPayload = {
  message: string
}

let unlisten: any
let unlistenModels: any

onMount(async () => {
  unlisten = listen('system_message', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event)
    const res = event.payload as TPayload

    if (res.message === 'Llama activated...') {
      LLMState.update((prev) => {
        return { ...prev, isRunning: true }
      })
      alert(res.message)
    }
  })

  unlistenModels = listen('models', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event)
    console.log(event.payload)
    const { is_running, running_model, local_models } = event.payload as any

    LLMState.update((prev) => {
      console.log('123')
      return {
        ...prev,
        isRunning: is_running,
        runnningModel: running_model,
        localModels: local_models
      }
    })
  })

  await invoke('update_llm_models').catch((e) => console.log(e))
})

onDestroy(async () => {
  await unlisten.then(() => console.log('stop listening...'))
  await unlistenModels.then(() => console.log('stop listening...'))
})
</script>

<main class="container">
  <h1 class=" text-center text-xl p-3">Lobot. &#129302;</h1>
  <Output />
  <Input />
</main>
