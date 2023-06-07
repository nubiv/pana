<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { LLMState } from '$lib/store/llm'
import { output } from '$lib/store/output'

let unlistenSys: UnlistenFn
let unlistenModel: UnlistenFn
let unlistenNoticification: UnlistenFn
let unlistenResponse: UnlistenFn
let unlistenError: UnlistenFn
let unlistenDownload: UnlistenFn

onMount(async () => {
  //   unlistenSys = await listen('system_message', (event) => {
  //     const res = event.payload as any

  //     if (res.message === 'Llama activated...') {
  //       LLMState.update((prev) => {
  //         return { ...prev, isRunning: true }
  //       })
  //       alert(res.message)
  //     }
  //   })

  unlistenModel = await listen('model', (event) => {
    const { running_model, local_models } = event.payload as any

    LLMState.update((prev) => {
      return {
        ...prev,
        runnningModel: running_model,
        localModels: local_models
      }
    })
  })

  unlistenNoticification = await listen('notification', (event) => {
    const res = event.payload as any

    if (res.message === 'Llama activated...') {
      LLMState.update((prev) => {
        return { ...prev, isRunning: true }
      })

      alert(res.message)
    }
  })

  unlistenResponse = await listen('response', (event) => {
    const res = event.payload as any
    console.log('start listening...')
    output.update((pre) => `${pre}\nLlama: ${res.message}`)
  })

  unlistenDownload = await listen('download', (event) => {
    console.log('download>>>', event)
  })

  unlistenError = await listen('error', (event) => {
    console.log('error>>>', event)
  })

  await invoke('update_llm_models').catch((e) => console.log(e))
})

onDestroy(() => {
  //   unlistenSys()
  unlistenModel()
  unlistenDownload()
  unlistenError()
  unlistenNoticification()
  unlistenResponse()
})
</script>

<slot />
