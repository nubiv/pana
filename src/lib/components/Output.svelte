<script lang="ts">
import { Container } from '@svelteuidev/core'
</script>

<Container
  override="{{
    bc: 'AliceBlue',
    whiteSpace: 'pre-wrap'
  }}">
  <slot />
</Container>
