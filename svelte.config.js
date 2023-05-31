import adapter from '@sveltejs/adapter-static' // This was changed from adapter-auto
import preprocess from 'svelte-preprocess'
import { vitePreprocess } from '@sveltejs/kit/vite'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    vitePreprocess(),
    preprocess({
      postcss: true
    })
  ],

  kit: {
    adapter: adapter(),
    alias: {
      '$lib/*': 'src/lib/*',
      '$componnets/*': 'src/lib/components/*',
      '$store/*': 'src/lib/components/store/*'
    }
  }
}

export default config
