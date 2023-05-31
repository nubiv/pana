<script lang="ts">
import '../app.postcss'
import { Label } from '$components/ui/label'
import { Switch } from '$components/ui/switch'
import { themeStore } from '$lib/store/theme'
import { Toggle } from '$lib/components/ui/toggle'
import { Sun } from 'lucide-svelte'
</script>

<!-- 
<div class="fixed right-0 p-3 items-center space-x-2">
  <Switch id="theme" on:change="{themeStore.toggleTheme}" />
  <Label for="theme">Dark Mode</Label>
</div> -->
<slot />

<Toggle class="fixed top-0 right-0 m-2" aria-label="toggle bold">
  <Sun class="h-4 w-4" />
</Toggle>

<button class="fixed top-0 left-0 m-2" on:click="{themeStore.toggleDarkMode}"
  >click me</button>
