<script lang="ts">
import { Button } from '$components/ui/button'
import { Label } from '$components/ui/label'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '$components/ui/sheet'
import { Settings } from 'lucide-svelte'
import Toggle from './ui/toggle/Toggle.svelte'
import Collapsible from './Collapsible.svelte'
import { LLMState } from '$lib/store/llm'
</script>

<Sheet>
  <SheetTrigger class="fixed top-0 right-0 m-2">
    <Toggle>
      <Settings class="h-4 w-4" />
    </Toggle>
  </SheetTrigger>
  <SheetContent position="right" size="sm">
    <SheetHeader>
      <SheetTitle>Settings</SheetTitle>
    </SheetHeader>
    <p class="w-auto space-y-2 pt-5">Currently running:<br /> Llama</p>
    <Collapsible title="Local Models" list="{$LLMState.localModels}" />
    <Collapsible title="Available Models" list="{[]}" />
    <Collapsible title="Download" list="{['downloading 1', 'downloading 2']}" />
  </SheetContent>
</Sheet>
