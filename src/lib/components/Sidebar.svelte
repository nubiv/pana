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
      <SheetDescription>
        Make changes to your settings here. Click save when you're done.
      </SheetDescription>
    </SheetHeader>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-right">Name</Label>
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="username" class="text-right">Username</Label>
      </div>
    </div>
    <SheetFooter>
      <SheetClose>
        <Button type="submit">Save changes</Button>
      </SheetClose>
    </SheetFooter>
  </SheetContent>
</Sheet>
