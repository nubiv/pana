<script lang="ts">
import { Tabs } from '@svelteuidev/core'
import { EnvelopeClosed, Gear, Person } from 'radix-icons-svelte'
import MessageWrapper from './MessageWrapper.svelte'
import Output from './Output.svelte'
</script>

<Tabs position="center">
  <Tabs.Tab label="Alpaca" icon="{Person}">
    <MessageWrapper />
    <Output />
  </Tabs.Tab>
  <Tabs.Tab label="Article Summary" icon="{EnvelopeClosed}"
    >Provide your article!</Tabs.Tab>
  <Tabs.Tab icon="{Gear}">Settings</Tabs.Tab>
</Tabs>
