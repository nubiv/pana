<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { LLMState } from '$lib/store/llm'
import { HistoryState, StreamState, type TMessage } from '$lib/store/history'
import { DownloadState } from '$lib/store/download'

let unlistenModel: UnlistenFn
let unlistenNoticification: UnlistenFn
let unlistenResponse: UnlistenFn
let unlistenError: UnlistenFn
let unlistenDownload: UnlistenFn

onMount(async () => {
  unlistenNoticification = await listen('notification', (event) => {
    type TNotificationPayload = {
      message: string
    }

    const payload = event.payload as TNotificationPayload
    console.log('Notification>>>', payload)
  })

  unlistenResponse = await listen('response', (event) => {
    type TResponsePayload = {
      is_streaming: boolean
      token: string
    }

    const payload = event.payload as TResponsePayload

    if (payload.is_streaming === true && !$StreamState.isStreaming) {
      StreamState.update((prev) => {
        return {
          ...prev,
          isStreaming: true
        }
      })
    }

    if (payload.token === '<|im_end|>') {
      HistoryState.update((prev) => {
        const newMessage: TMessage = {
          text: $StreamState.tokens,
          role: 'Lobot'
        }
        return [...prev, newMessage]
      })

      StreamState.update((prev) => {
        return {
          ...prev,
          isStreaming: false,
          tokens: ''
        }
      })
    } else {
      StreamState.update((prev) => {
        return {
          ...prev,
          tokens: prev.tokens.concat(payload.token)
        }
      })
    }
  })

  unlistenDownload = await listen('download', (event) => {
    type TDownloadPayload = {
      progress: number
    }

    let payload = event.payload as any

    DownloadState.update((prev) => {
      return {
        ...prev,
        progress: payload.progress
      }
    })

    if (payload.progress === '100.00') {
      DownloadState.update((prev) => {
        return {
          ...prev,
          currentDownload: null,
          progress: 0
        }
      })

      invoke('update_llm_models').catch((e) => console.log(e))
    }
  })

  unlistenError = await listen('error', (event) => {
    type TErrorPayload = {
      message: string
    }

    let payload = event.payload as TErrorPayload
    console.log('Error>>>', payload)
  })

  unlistenModel = await listen('model', (event) => {
    type TModelPayload = {
      name: string
      size: number
      total_size: number
    }

    const { name, size, total_size } = event.payload as TModelPayload

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
})

onDestroy(() => {
  unlistenModel()
  unlistenDownload()
  unlistenError()
  unlistenNoticification()
  unlistenResponse()
})
</script>

<slot />
