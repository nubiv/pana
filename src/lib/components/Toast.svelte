<script lang="ts">
import { Terminal } from 'lucide-svelte'
import { Alert, AlertDescription, AlertTitle } from '$components/ui/alert'
import { toasts } from '$lib/store/toasts'
</script>

<div class="fixed w-1/3 bottom-1 right-1 z-[9999]">
  {#each $toasts as toast}
    <Alert class="animate-in animate-out m-1">
      <Terminal class="h-4 w-4" />
      <AlertTitle>{toast.type}</AlertTitle>
      <AlertDescription>
        {toast.message}
      </AlertDescription>
    </Alert>
  {/each}
</div>
