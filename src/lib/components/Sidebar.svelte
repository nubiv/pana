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
import { LLMState } from '$lib/store/llm'
import { resolveResource } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/tauri'

async function openModelFolder() {
  const path = await resolveResource('models')
  await invoke('open_model_folder', { path })
}
</script>

<Sheet>
  <SheetTrigger class="fixed top-0 right-0 m-2">
    <Toggle>
      <Settings class="h-4 w-4" />
    </Toggle>
  </SheetTrigger>
  <SheetContent position="right" size="lg">
    <SheetHeader>
      <SheetTitle>Settings</SheetTitle>
    </SheetHeader>
    <div class="w-auto space-y-2 pt-5">
      <span class=" font-medium space-y-2 mx-4 pt-3"> Currently running: </span>
      <span class="space-y-2 mx-4 pt-3">
        {$LLMState.runnningModel ? $LLMState.runnningModel : 'None'}
      </span>
    </div>
    <Collapsible title="Local Models" list="{$LLMState.localModels}" />
    <Collapsible
      title="Other Available Models"
      list="{$LLMState.otherModels}" />
    <Button class="mt-4 mr-7 float-right" on:click="{openModelFolder}"
      >View models</Button>
  </SheetContent>
</Sheet>
