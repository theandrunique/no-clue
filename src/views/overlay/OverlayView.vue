<script setup lang="ts">
import { onMounted, onUnmounted, computed } from "vue";
import { useOverlayStore } from "@/stores/overlay";
import { useSettingsStore } from "@/stores/settings";
import { Mic, MicOff, Settings, Camera, CameraOff } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import DragButton from "@/components/ui/drag-button/DragButton.vue";
import { Button } from "@/components/ui/button";
import OverlayPopover from "./components/OverlayPopover.vue";
import Card from "@/components/ui/card/Card.vue";

const overlayStore = useOverlayStore();
const settingsStore = useSettingsStore();

const cardStyle = computed(() => ({
  "--overlay-opacity": settingsStore.settings.overlayOpacity,
}));

onMounted(() => {
  document.body.classList.add("transparent");
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
            @click="overlayStore.setTranscriptionEnabled(!overlayStore.isTranscriptionEnabled)"
          >
            <Mic v-if="overlayStore.isTranscriptionEnabled" class="w-4 h-4" />
            <MicOff v-else class="w-4 h-4" />
          </Button>

          <Button
            variant="default"
            size="icon"
            @click="overlayStore.setCaptureScreenshot(!overlayStore.captureScreenshot)"
          >
            <Camera v-if="overlayStore.captureScreenshot" class="w-4 h-4" />
            <CameraOff v-else class="w-4 h-4" />
          </Button>
        </div>

        <div class="flex gap-2 items-center">
          <Button variant="ghost" size="icon" @click="openDashboard">
            <Settings :size="18" />
          </Button>

          <DragButton />
        </div>
    </Card>
  </div>
</template>
