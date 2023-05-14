<script lang="ts" context="module">
import { Input, Button, InputWrapper, Center } from '@svelteuidev/core'
import { invoke } from '@tauri-apps/api/tauri'
import { writable, type Writable } from 'svelte/store'

export const testMsg: Writable<string> = writable('Drop output here.')

async function test() {
  const res: string = await invoke('test')

  testMsg.update((msg) => msg + '\nNew message: ' + res)
}
</script>

<InputWrapper id="message-input" label="">
  <Input placeholder="Enter your message here" />
  <Center>
    <Button variant="light" on:click="{test}">Send Message</Button>
  </Center>
</InputWrapper>
