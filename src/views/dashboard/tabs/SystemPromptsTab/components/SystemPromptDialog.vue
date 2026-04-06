<script setup lang="ts">
import { ref, watch } from "vue";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Textarea } from "@/components/ui/textarea";

interface Props {
  open: boolean;
  isEditing: boolean;
  prompt?: {
    id: string;
    name: string;
    prompt: string;
  } | null;
}

interface Emits {
  (e: "close"): void;
  (e: "save", payload: { name: string; prompt: string }): void;
}

const props = defineProps<Props>();
const emits = defineEmits<Emits>();

const name = ref("");
const promptText = ref("");
const saving = ref(false);

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      name.value = props.prompt?.name ?? "";
      promptText.value = props.prompt?.prompt ?? "";
    }
  }
);

async function handleSave() {
  if (!name.value.trim() || !promptText.value.trim()) return;

  saving.value = true;
  try {
    emits("save", { name: name.value.trim(), prompt: promptText.value.trim() });
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="emits('close')">
    <DialogContent class="sm:max-w-[500px] dark:bg-background dark:text-foreground">
      <DialogHeader>
        <DialogTitle>
          {{ isEditing ? "Edit System Prompt" : "New System Prompt" }}
        </DialogTitle>
        <DialogDescription>
          {{ isEditing ? "Update the system prompt details." : "Create a new system prompt." }}
        </DialogDescription>
      </DialogHeader>

      <div class="grid gap-4 py-4">
        <div class="grid gap-2">
          <label for="name" class="text-sm font-medium">Name</label>
          <Input
            id="name"
            v-model="name"
            placeholder="My Custom Prompt"
            autocomplete="off"
          />
        </div>

        <div class="grid gap-2">
          <label for="prompt" class="text-sm font-medium">Prompt</label>
          <Textarea
            id="prompt"
            v-model="promptText"
            placeholder="You are a helpful assistant..."
            rows="6"
            autocomplete="off"
          />
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="emits('close')">Cancel</Button>
        <Button
          :disabled="!name.trim() || !promptText.trim() || saving"
          @click="handleSave"
        >
          {{ saving ? "Saving..." : "Save" }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
