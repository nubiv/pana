<script lang="ts">
import { HistoryState } from '$lib/store/history'
import { cn } from '$lib/utils'
import { afterUpdate } from 'svelte'

let className: string | undefined | null = undefined
export { className as class }

let element: HTMLDivElement
const scrollToBottom = async (node: HTMLDivElement) => {
  node.scroll({ top: node.scrollHeight, behavior: 'smooth' })
}

afterUpdate(() => {
  if (!element || !$HistoryState) return
  scrollToBottom(element)
})
</script>

<div
  bind:this="{element}"
  class="{cn(
    'rounded-lg border bg-card text-card-foreground shadow-sm',
    className
  )}"
  {...$$restProps}
  on:click
  on:focusin
  on:focusout
  on:mouseenter
  on:mouseleave>
  <slot />
</div>
