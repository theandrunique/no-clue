<script setup lang="ts">
import { ref } from "vue";
import { onMounted } from "vue";
import { storeToRefs } from "pinia";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-vue-next";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useSystemPromptsStore } from "@/stores/systemPrompts";
import { type SystemPrompt } from "@/types";
import SystemPromptCard from "./components/SystemPromptCard.vue";
import SystemPromptDialog from "./components/SystemPromptDialog.vue";

const store = useSystemPromptsStore();
const { prompts, activePromptId } = storeToRefs(store);

const dialogOpen = ref(false);
const editingPrompt = ref<SystemPrompt | null>(null);

const deleteConfirmOpen = ref(false);
const pendingDeleteId = ref<string | null>(null);

function openCreateDialog() {
  editingPrompt.value = null;
  dialogOpen.value = true;
}

function openEditDialog(prompt: SystemPrompt) {
  editingPrompt.value = prompt;
  dialogOpen.value = true;
}

async function handleSave(payload: { name: string; prompt: string }) {
  try {
    if (editingPrompt.value) {
      await store.updatePrompt(editingPrompt.value.id, payload);
    } else {
      await store.createPrompt(payload);
    }
    dialogOpen.value = false;
    editingPrompt.value = null;
  } catch (e) {
    console.error("Failed to save prompt:", e);
  }
}

function handleRequestDelete(id: string) {
  pendingDeleteId.value = id;
  deleteConfirmOpen.value = true;
}

async function confirmDelete() {
  if (pendingDeleteId.value) {
    await store.deletePrompt(pendingDeleteId.value);
    deleteConfirmOpen.value = false;
    pendingDeleteId.value = null;
  }
}

function handleSelect(prompt: SystemPrompt) {
  store.setActive(prompt.id);
}

onMounted(() => {
  store.loadPrompts();
});
</script>

<template>
  <div class="flex flex-col h-full overflow-hidden">
    <div class="p-4 border-b border-border flex items-center justify-between shrink-0">
      <h3 class="text-sm font-medium">System Prompts</h3>
      <Button size="sm" @click="openCreateDialog">
        <Plus class="w-4 h-4 mr-1" />
        Create New
      </Button>
    </div>

    <div class="flex-1 overflow-y-auto p-4">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <SystemPromptCard
          v-for="prompt in prompts"
          :key="prompt.id"
          :prompt="prompt"
          :is-selected="activePromptId === prompt.id"
          @select="handleSelect"
          @edit="openEditDialog"
          @request-delete="handleRequestDelete"
        />
        <div
          v-if="prompts.length === 0"
          class="col-span-full flex items-center justify-center text-muted-foreground text-sm py-8"
        >
          No system prompts yet
        </div>
      </div>
    </div>

    <SystemPromptDialog
      :open="dialogOpen"
      :prompt="editingPrompt"
      :is-editing="editingPrompt !== null"
      @close="dialogOpen = false; editingPrompt = null"
      @save="handleSave"
    />

    <Dialog :open="deleteConfirmOpen" @update:open="deleteConfirmOpen = false">
      <DialogContent class="sm:max-w-[400px] dark:bg-background dark:text-foreground">
        <DialogHeader>
          <DialogTitle>Delete System Prompt</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete this prompt? This action cannot be undone.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" @click="deleteConfirmOpen = false">Cancel</Button>
          <Button variant="destructive" @click="confirmDelete">Delete</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>