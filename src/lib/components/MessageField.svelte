<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { HistoryState, StreamState, type TMessage } from '../store/history'
import { LLMState } from '../store/llm'
import { Input } from '$components/ui/input'
import { Button } from '$components/ui/button'
import { CornerDownLeft } from 'lucide-svelte'
import { toasts } from '$lib/store/toasts'

let message: string

async function sendMessage() {
  if (!message) {
    toasts.error('Message is empty...')
    return
  }

  if (!$LLMState.runnningModel) {
    toasts.error('Activate model first...')
    message = ''
    return
  }

  if ($StreamState.isStreaming) {
    toasts.error('Still processing wait a sec...')
    message = ''
    return
  }

  await invoke('start_inference', { message: message })
  const myMessage: TMessage = { text: message, role: 'Me' }
  HistoryState.addMessage(myMessage)

  message = ''
}
</script>

<div class="w-[80%] mx-auto">
  <Input
    type="message"
    placeholder="Press enter to send..."
    bind:value="{message}"
    on:keydown="{(e) => e.key === 'Enter' && sendMessage()}" />
  <!-- <CornerDownLeft class="fixed right-2" /> -->
</div>
