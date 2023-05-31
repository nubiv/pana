<script lang="ts">
import { AspectRatio } from 'radix-svelte'
import type { ResolvedProps } from 'radix-svelte/internal/helpers'

let rootRatio = 1.75
</script>

<div class="w-[300px] overflow-hidden rounded-md shadow-lg">
  <AspectRatio.Root ratio="{rootRatio}">
    <img
      class="h-full w-full object-cover"
      src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
      alt="Landscape photograph by Tobias Tullius" />
  </AspectRatio.Root>
</div>
