<script setup lang="ts">
import { useOverlayStore } from "@/stores/overlay";
import { Mic, MicOff, Settings, Camera, CameraOff } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import DragButton from "@/components/ui/drag-button/DragButton.vue";
import { Button } from "@/components/ui/button";
import OverlayPopover from "./components/OverlayPopover.vue";
import Card from "@/components/ui/card/Card.vue";

const overlayStore = useOverlayStore();

async function openDashboard() {
  await invoke("open_dashboard");
}

</script>

<template>
  <div class="dark w-screen h-screen flex overflow-hidden justify-center items-start">
    <Card class="w-full flex flex-row items-center justify-between gap-2 p-2">
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
        </div>

        <div class="flex gap-2 items-center">
          <Button
            variant="default"
            size="icon"
            @click="overlayStore.setCaptureScreenshot(!overlayStore.captureScreenshot)"
          >
            <Camera v-if="overlayStore.captureScreenshot" class="w-4 h-4" />
            <CameraOff v-else class="w-4 h-4" />
          </Button>

          <Button variant="ghost" size="icon" @click="openDashboard">
            <Settings :size="18" />
          </Button>

          <DragButton />
        </div>
    </Card>
  </div>
</template>
