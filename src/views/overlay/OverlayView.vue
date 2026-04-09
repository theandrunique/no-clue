<script setup lang="ts">
import { onMounted, onUnmounted, computed } from "vue";
import { useSettingsStore } from "@/stores/settings";
import { useConversationStore } from "@/stores/conversation";
import { useChatStore } from "@/stores/chat";
import { Mic, MicOff, Settings, Camera, CameraOff, Trash2 } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import DragButton from "@/components/ui/drag-button/DragButton.vue";
import { Button } from "@/components/ui/button";
import OverlayPopover from "./components/OverlayPopover.vue";
import ContextIndicator from "./components/ContextIndicator.vue";
import Card from "@/components/ui/card/Card.vue";

const settingsStore = useSettingsStore();
const conversationStore = useConversationStore();
const chatStore = useChatStore();

const cardStyle = computed(() => ({
  "--overlay-opacity": settingsStore.settings.overlayOpacity,
}));

function toggleMic() {
  conversationStore.toggleTranscription();
}

function toggleScreenshot() {
  if (!chatStore.canUseScreenshot && !chatStore.isScreenshotEnabled) {
    return;
  }
  conversationStore.setCaptureScreenshot(!conversationStore.isScreenshotEnabled);
}

onMounted(async () => {
  document.body.classList.add("transparent");
  await chatStore.loadModelInfo();
});

onUnmounted(() => {
  document.body.classList.remove("transparent");
});

async function openDashboard() {
  await invoke("open_dashboard");
}
</script>

<template>
  <div class="dark w-screen h-screen flex overflow-hidden justify-center items-start">
    <Card :style="cardStyle" class="w-full flex flex-row items-center justify-between gap-2 p-2 overlay-card">
        <div class="flex gap-2 items-center">
          <OverlayPopover />

          <Button
            variant="default"
            size="icon"
            @click="toggleMic"
          >
            <Mic v-if="conversationStore.isTranscriptionEnabled" class="w-4 h-4" />
            <MicOff v-else class="w-4 h-4" />
          </Button>

          <Button
            variant="default"
            size="icon"
            :class="{ 'opacity-50 cursor-not-allowed': !chatStore.canUseScreenshot }"
            :disabled="!chatStore.canUseScreenshot && !conversationStore.isScreenshotEnabled"
            @click="toggleScreenshot"
          >
            <Camera v-if="conversationStore.isScreenshotEnabled" class="w-4 h-4" />
            <CameraOff v-else class="w-4 h-4" />
          </Button>
        </div>

        <div class="flex gap-2 items-center">
          <ContextIndicator />
            <Button
                variant="ghost"
                size="icon"
                @click="conversationStore.clearCurrentConversation()"
            >
                <Trash2 class="w-4 h-4" />
            </Button>

          <Button variant="ghost" size="icon" @click="openDashboard">
            <Settings class="w-4 h-4" />
          </Button>

          <DragButton />
        </div>
    </Card>
  </div>
</template>
