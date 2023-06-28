<script lang="ts">
import { Button } from '$components/ui/button'
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '$components/ui/sheet'
import { Settings } from 'lucide-svelte'
import Toggle from './ui/toggle/Toggle.svelte'
import Collapsible from './Collapsible.svelte'
import { LLMState, LocalModels, OtherModels, type TModel } from '$lib/store/llm'
import { invoke } from '@tauri-apps/api/tauri'
import { toasts } from '$lib/store/toasts'
import { HistoryState } from '$lib/store/history'

async function openModelFolder() {
  await invoke('open_model_folder').catch((err) => {
    toasts.error(err)
  })
}

async function clearHistory() {
  await invoke('clear_history').catch((err) => {
    toasts.error(err)
  })

  HistoryState.clearHistory()
}
</script>

<Sheet>
  <SheetTrigger class="fixed top-0 right-0 m-2">
    <Toggle>
      <Settings class="h-4 w-4" />
    </Toggle>
  </SheetTrigger>
  <SheetContent class="overflow-auto no-scrollbar" position="right" size="lg">
    <SheetHeader>
      <SheetTitle>Settings</SheetTitle>
    </SheetHeader>
    <div class="w-auto space-y-2 pt-5">
      <span class=" font-medium space-y-2 mx-4 pt-3"> Currently running: </span>
      <span class="space-y-2 mx-4 pt-3">
        {$LLMState.runnningModel ? $LLMState.runnningModel : 'None'}
      </span>
    </div>
    <Collapsible title="Local Models" list="{$LocalModels}" />
    <Collapsible title="Other Available Models" list="{$OtherModels}" />
    <Button
      variant="outline"
      class="mt-4 mr-7 float-right"
      on:click="{openModelFolder}">View models</Button>
    <Button
      variant="outline"
      class="mt-4 mr-7 float-right"
      on:click="{clearHistory}">Clear history</Button>
  </SheetContent>
</Sheet>
