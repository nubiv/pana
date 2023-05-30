<script lang="ts">
import { Input, Button, InputWrapper, Center } from '@svelteuidev/core'
import { invoke } from '@tauri-apps/api/tauri'
import { output } from '../store/output'
import { llmState } from '../store/llm-state'

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

<InputWrapper id="message-input" label="">
  <Input placeholder="Enter your message here" bind:value="{message}" />
  <Center>
    <Button variant="light" on:click="{sendMessage}">Send Message</Button>
    <Button variant="light" on:click="{runLlama}">Run Llama</Button>
  </Center>
</InputWrapper>
