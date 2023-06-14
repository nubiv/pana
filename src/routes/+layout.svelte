<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { LLMState, type TLocalModel, type TOtherModel } from '$lib/store/llm'
import { output } from '$lib/store/output'
import { DownloadState } from '$lib/store/download'

let unlistenModel: UnlistenFn
let unlistenNoticification: UnlistenFn
let unlistenResponse: UnlistenFn
let unlistenError: UnlistenFn
let unlistenDownload: UnlistenFn
let unlistenModelV2: UnlistenFn

onMount(async () => {
  unlistenModel = await listen('model', (event) => {
    // const { running_model, local_models } = event.payload as any
    // LLMState.update((prev) => {
    //   return {
    //     ...prev,
    //     runnningModel: running_model,
    //     localModels: local_models
    //   }
    // })
  })

  unlistenNoticification = await listen('notification', (event) => {
    const res = event.payload as any

    if (res.message === 'Llama activated...') {
      LLMState.update((prev) => {
        return { ...prev, isRunning: true }
      })
    }

    alert(res.message)
  })

  unlistenResponse = await listen('response', (event) => {
    const res = event.payload as any
    console.log('start listening...')
    output.update((pre) => `${pre}\nLobot: ${res.message}`)
  })

  unlistenDownload = await listen('download', (event) => {
    // console.log('download>>>', event)
    console.log('download>>>', event.payload)
    DownloadState.update((prev) => {
      return {
        ...prev,
        progress: (event.payload as any).progress
      }
    })

    if ((event.payload as any).progress === '100.00') {
      DownloadState.update((prev) => {
        return {
          ...prev,
          currentDownload: null,
          progress: 0
        }
      })

      invoke('update_llm_models_v2').catch((e) => console.log(e))
    }
  })

  unlistenError = await listen('error', (event) => {
    console.log('error>>>', event)
  })

  unlistenModelV2 = await listen('model_v2', (event) => {
    console.log('model_v2>>>', event)

    const { name, size, total_size } = event.payload as any

    if (size === total_size) {
      LLMState.update((prev) => {
        return {
          ...prev,
          localModels: {
            ...prev.localModels,
            [name]: {
              name,
              size,
              totalSize: total_size
            }
          }
        }
      })
    } else {
      LLMState.update((prev) => {
        return {
          ...prev,
          otherModels: {
            ...prev.otherModels,
            [name]: {
              name,
              size,
              totalSize: total_size
            }
          }
        }
      })
    }
  })

  await invoke('update_llm_models').catch((e) => console.log(e))
  await invoke('update_llm_models_v2').catch((e) => console.log(e))
})

onDestroy(() => {
  unlistenModel()
  unlistenDownload()
  unlistenError()
  unlistenNoticification()
  unlistenResponse()
  unlistenModelV2()
})
</script>

<slot />
