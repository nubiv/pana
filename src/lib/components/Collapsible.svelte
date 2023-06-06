<script lang="ts">
import { ChevronDown } from 'lucide-svelte'
import { Button } from '$components/ui/button'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger
} from '$components/ui/collapsible'

let isOpen = false
export let list: any[]
export let title: string
</script>

<Collapsible open="{isOpen}" class="w-auto space-y-2 pt-5">
  <div class="flex items-center justify-between space-x-4 px-4">
    <h4 class="text-sm font-semibold">{title}</h4>
    <CollapsibleTrigger>
      <Button variant="ghost" size="sm" class="w-9 p-0">
        <ChevronDown class="h-4 w-4" />
        <span class="sr-only">Toggle</span>
      </Button>
    </CollapsibleTrigger>
  </div>
  <CollapsibleContent class="space-y-2">
    {#if list.length == 0}
      <div class="rounded-md overflow-auto border px-4 py-3 font-mono text-sm">
        None
      </div>
    {:else}
      {#each list as item}
        <div
          class="rounded-md overflow-auto border px-4 py-3 font-mono text-sm">
          {item}
        </div>
      {/each}
    {/if}
  </CollapsibleContent>
</Collapsible>
