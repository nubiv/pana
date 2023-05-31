<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { output } from '../store/output'
import { llmState } from '../store/llm-state'
import { Input } from '$components/ui/input'
import { Button } from '$components/ui/button'

let message: string

async function sendMessage() {
  if (!message) {
    alert('Message is empty...')
    return
  }

  if (!$llmState) {
    alert('Wake Llama up first...')
    return
  }

  await invoke('send_message', { message: message })
  output.update((prev) => `${prev}\nMe: ${message}`)
}

async function runLlama() {
  await invoke('run_llama')
}
</script>

<Input
  type="message"
  placeholder="Enter your message..."
  bind:value="{message}" />
<Button on:click="{sendMessage}">Send</Button>
<Button on:click="{runLlama}">Run llama</Button>
