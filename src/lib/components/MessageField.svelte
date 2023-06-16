<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { HistoryState, type TMessage } from '../store/history'
import { LLMState } from '../store/llm'
import { Input } from '$components/ui/input'
import { Button } from '$components/ui/button'
import { CornerDownLeft } from 'lucide-svelte'

let message: string

async function sendMessage() {
  if (!message) {
    alert('Message is empty...')
    return
  }

  // if (!$LLMState.isRunning) {
  //   alert('Wake Lobot up first...')
  //   return
  // }

  await invoke('start_inference', { message: message })
  // output.update((prev) => `${prev}\nMe: ${message}`)
  HistoryState.update((prev) => {
    const myMessage: TMessage = { text: message, role: 'Me' }
    return [...prev, myMessage]
  })

  message = ''
}

async function runLlama() {
  await invoke('run_llama')
}

async function llmTest() {
  await invoke('llm_test', { message: message })
}

async function stopLlama() {
  await invoke('stop_llama')
}

async function download() {
  await invoke('download_model')
}

async function sendMessageV2() {
  await invoke('send_message_v2', { message: message })

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
<!-- <Button on:click="{sendMessage}">Send</Button> -->
<!-- <Button on:click="{runLlama}">Activate Lobot</Button>
<Button on:click="{llmTest}">Test LLM Version</Button> -->
<!-- <Button on:click="{stopLlama}">Shut Lobot</Button>
<Button on:click="{download}">Download</Button> -->
