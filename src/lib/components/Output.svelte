<style>
p {
  white-space: pre-line;
}
</style>

<script lang="ts">
import { output } from '$store/output'
import { AspectRatio } from './ui/aspect-ratio'
</script>

<AspectRatio
  ratio="{16 / 9}"
  class="bg-muted flex flex-col-reverse overflow-scroll ">
  <p>
    {$output}
  </p>
</AspectRatio>
