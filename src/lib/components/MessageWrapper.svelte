<script lang="ts" context="module">
import { Input, Button, InputWrapper, Center } from '@svelteuidev/core'
import { invoke } from '@tauri-apps/api/tauri'
import { writable, type Writable } from 'svelte/store'

export const testMsg: Writable<string> = writable('Drop output here.')
</script>

<InputWrapper id="message-input" label="">
  <Input placeholder="Enter your message here" />
  <Center>
    <Button variant="light">Send Message</Button>
  </Center>
</InputWrapper>
