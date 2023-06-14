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
  <SheetContent position="right" size="lg">
    <SheetHeader>
      <SheetTitle>Settings</SheetTitle>
    </SheetHeader>
    <div class="w-auto space-y-2 pt-5">
      <span class="space-y-2 mx-4 pt-3"> Currently running: </span>
      <span class="space-y-2 mx-4 pt-3"> XXX </span>
    </div>
    <Collapsible title="Local Models" list="{$LLMState.localModels}" />
    <Collapsible
      title="Other Available Models"
      list="{$LLMState.otherModels}" />
    <!-- <Collapsible title="Download" list="{['downloading 1', 'downloading 2']}" /> -->
  </SheetContent>
</Sheet>
