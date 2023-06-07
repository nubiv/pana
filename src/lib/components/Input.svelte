<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { output } from '../store/output'
import { LLMState } from '../store/llm'
import { Input } from '$components/ui/input'
import { Button } from '$components/ui/button'

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

  await invoke('send_message', { message: message })
  output.update((prev) => `${prev}\nMe: ${message}`)

  message = ''
}

async function runLlama() {
  await invoke('run_llama')
}

async function stopLlama() {
  await invoke('stop_llama')
}

async function download() {
  await invoke('download_model')
}
</script>

<Input
  type="message"
  placeholder="Enter your message..."
  bind:value="{message}"
  on:keydown="{(e) => e.key === 'Enter' && sendMessage()}" />
<Button on:click="{sendMessage}">Send</Button>
<Button on:click="{runLlama}">Activate Lobot</Button>
<!-- <Button on:click="{stopLlama}">Shut Lobot</Button>
<Button on:click="{download}">Download</Button> -->
