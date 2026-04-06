<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
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
import { type SystemPrompt } from "@/types";
import SystemPromptCard from "./components/SystemPromptCard.vue";
import SystemPromptDialog from "./components/SystemPromptDialog.vue";

const prompts = ref<SystemPrompt[]>([]);
const selectedPrompt = ref<SystemPrompt | null>(null);
const dialogOpen = ref(false);
const editingPrompt = ref<SystemPrompt | null>(null);
const loading = ref(false);

const ACTIVE_PROMPT_KEY = "active_system_prompt_id";
const activePromptId = ref<string | null>(null);

const deleteConfirmOpen = ref(false);
const pendingDeleteId = ref<string | null>(null);

function getActivePromptId(): string | null {
  return localStorage.getItem(ACTIVE_PROMPT_KEY);
}

async function loadPrompts() {
  loading.value = true;
  try {
    prompts.value = await invoke<SystemPrompt[]>("get_system_prompts");
    activePromptId.value = getActivePromptId();
  } catch (e) {
    console.error("Failed to load prompts:", e);
  } finally {
    loading.value = false;
  }
}

async function handleSave(payload: { name: string; prompt: string }) {
  try {
    if (editingPrompt.value) {
      await invoke("update_system_prompt", {
        id: editingPrompt.value.id,
        name: payload.name,
        prompt: payload.prompt,
      });
    } else {
      const newPrompt = await invoke<SystemPrompt>("create_system_prompt", {
        name: payload.name,
        prompt: payload.prompt,
      });
      prompts.value.unshift(newPrompt);
    }
    await loadPrompts();
    dialogOpen.value = false;
    editingPrompt.value = null;
  } catch (e) {
    console.error("Failed to save prompt:", e);
  }
}

async function handleDelete(id: string) {
  if (!id) return;
  try {
    await invoke("delete_system_prompt", { id });
    prompts.value = prompts.value.filter((p) => p.id !== id);
    if (selectedPrompt.value?.id === id) {
      selectedPrompt.value = null;
    }
    if (activePromptId.value === id) {
      activePromptId.value = null;
      localStorage.removeItem(ACTIVE_PROMPT_KEY);
    }
    deleteConfirmOpen.value = false;
    pendingDeleteId.value = null;
  } catch (e) {
    console.error("Failed to delete prompt:", e);
  }
}

function handleSelect(prompt: SystemPrompt) {
  selectedPrompt.value = prompt;
}

function openCreateDialog() {
  editingPrompt.value = null;
  dialogOpen.value = true;
}

function openEditDialog(prompt: SystemPrompt) {
  editingPrompt.value = prompt;
  dialogOpen.value = true;
}

function handleRequestDelete(id: string) {
  pendingDeleteId.value = id;
  deleteConfirmOpen.value = true;
}

onMounted(() => {
  loadPrompts();
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
          :is-selected="selectedPrompt?.id === prompt.id || activePromptId === prompt.id"
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
          <Button variant="destructive" @click="handleDelete(pendingDeleteId!)">Delete</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>