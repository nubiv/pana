<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type UnlistenFn, type Event } from '@tauri-apps/api/event'
import { onDestroy, onMount } from 'svelte'
import { LLMState, type TModel } from '$lib/store/llm'
import { HistoryState, StreamState, type TMessage } from '$lib/store/history'
import { DownloadState } from '$lib/store/download'
import Toast from '$lib/components/Toast.svelte'
import { toasts } from '$lib/store/toasts'

let unlistenModel: UnlistenFn
let unlistenNoticification: UnlistenFn
let unlistenResponse: UnlistenFn
let unlistenError: UnlistenFn
let unlistenDownload: UnlistenFn

type TNotificationPayload = {
  message: string
}

type TResponsePayload = {
  is_streaming: boolean
  is_feeding_prompt: boolean
  token: string
}

type TDownloadPayload = {
  size: number
}

type TErrorPayload = {
  message: string
}

type TModelPayload = {
  name: string
  size: number
  total_size: number
}

onMount(async () => {
  unlistenNoticification = await listen(
    'notification',
    (event: Event<TNotificationPayload>) => {
      const message = event.payload.message
      toasts.notifications(message)
    }
  )

  unlistenResponse = await listen(
    'response',
    (event: Event<TResponsePayload>) => {
      const payload = event.payload

      if (payload.is_streaming && !$StreamState.isStreaming) {
        StreamState.startStream()
        return
      }

      if (payload.is_feeding_prompt) {
        StreamState.feedPrompt()
        return
      }

      if (!payload.is_streaming) {
        StreamState.stopStream()
      } else {
        StreamState.setTokens(payload.token)
      }
    }
  )

  unlistenDownload = await listen(
    'download',
    async (event: Event<TDownloadPayload>) => {
      let payload = event.payload

      DownloadState.updateProgress(payload)

      if (payload.size === $DownloadState.totalSize) {
        DownloadState.stopDownload()

        await invoke('update_llm_models').catch((e) => console.log(e))
      }
    }
  )

  unlistenError = await listen('error', (event: Event<TErrorPayload>) => {
    const payload = event.payload
    toasts.error(payload.message)
  })

  unlistenModel = await listen('model', (event: Event<TModelPayload>) => {
    const payload = event.payload
    LLMState.updateModelList(payload)
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
<Toast />
