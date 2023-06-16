<script lang="ts">
import { Toggle } from '$lib/components/ui/toggle'
import { themeStore } from '$lib/store/theme'
import { Sun } from 'lucide-svelte'
import { afterUpdate } from 'svelte'

let pressed: boolean

afterUpdate(() => {
  if (pressed) {
    themeStore.toggleLightMode()
  } else {
    themeStore.toggleDarkMode()
  }
})
</script>

<Toggle
  class="fixed top-0 right-[45px] m-2"
  bind:pressed="{pressed}"
  aria-label="toggle theme">
  <Sun class="h-4 w-4" />
</Toggle>
