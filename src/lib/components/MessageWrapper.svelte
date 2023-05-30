<script lang="ts" context="module">
import { Input, Button, InputWrapper, Center } from '@svelteuidev/core'
import { invoke } from '@tauri-apps/api/tauri'

let message: string

async function sendMessage() {
  if (!message) return
  await invoke('send_message', { message: message })
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
