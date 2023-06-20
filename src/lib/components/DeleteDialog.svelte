<script lang="ts">
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger
} from '$components/ui/alert-dialog'
import { Button } from '$components/ui/button'
import { DownloadState } from '$lib/store/download'
import { LLMState } from '$lib/store/llm'
import { toasts } from '$lib/store/toasts'
import { invoke } from '@tauri-apps/api/tauri'
import { Trash2 } from 'lucide-svelte'

export let modelName: string

async function deleteModel() {
  if ($DownloadState.currentDownload === modelName) {
    toasts.error('Stop downloading first...')
    return
  }

  if ($LLMState.runnningModel === modelName) {
    toasts.error('Stop running it first...')
    return
  }

  await invoke('delete_model', { modelName })

  await invoke('update_llm_models')
}
</script>

<AlertDialog>
  <AlertDialogTrigger>
    <Button variant="ghost" size="sm" class="px-1 group" id="{modelName}">
      <Trash2 class="h-4 w-4 pointer-events-none" />
      <span class="sr-only pointer-events-none">Delete</span>
    </Button>
  </AlertDialogTrigger>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
      <AlertDialogDescription>
        This action cannot be undone. This will delete selected model.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction>
        <button on:click="{deleteModel}">Continue</button>
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
