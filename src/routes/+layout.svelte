<script lang="ts">
import '../app.postcss'
import Sidebar from '$lib/components/Sidebar.svelte'
import ThemeToggler from '$lib/components/ThemeToggler.svelte'
</script>

<ThemeToggler />
<Sidebar />

<slot />
