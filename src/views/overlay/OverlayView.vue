<script setup lang="ts">
import { useOverlayStore } from "../../stores/overlay";
import { ChevronUp, ChevronDown, Mic, MicOff, Settings } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";

const overlayStore = useOverlayStore();

async function openDashboard() {
  await invoke("open_dashboard");
}
</script>

<template>
  <div class="overlay-container">
    <!-- Draggable Header / Toolbar -->
    <div 
      class="overlay-toolbar" 
      data-tauri-drag-region
    >
      <div class="toolbar-left" data-tauri-drag-region>
        <button 
          class="toolbar-btn" 
          @click="overlayStore.toggleExpanded"
          :title="overlayStore.isExpanded ? 'Collapse' : 'Expand'"
        >
          <ChevronDown v-if="!overlayStore.isExpanded" :size="18" />
          <ChevronUp v-else :size="18" />
        </button>
        
        <button 
          class="toolbar-btn"
          :class="{ 'active': overlayStore.isTranscriptionEnabled }"
          @click="overlayStore.setTranscriptionEnabled(!overlayStore.isTranscriptionEnabled)"
          title="Toggle Transcription"
        >
          <Mic v-if="overlayStore.isTranscriptionEnabled" :size="18" />
          <MicOff v-else :size="18" />
        </button>
      </div>
      
      <div class="toolbar-right">
        <button 
          class="toolbar-btn"
          @click="overlayStore.setCaptureScreenshot(!overlayStore.captureScreenshot)"
          :class="{ 'active': overlayStore.captureScreenshot }"
          title="Capture Screenshot"
        >
          📷
        </button>
        
        <button 
          class="toolbar-btn"
          @click="openDashboard"
          title="Open Dashboard"
        >
          <Settings :size="18" />
        </button>
      </div>
    </div>
    
    <!-- Expandable Content -->
    <div v-if="overlayStore.isExpanded" class="overlay-content">
      <p class="text-white p-4">Overlay Content - to be implemented</p>
    </div>
  </div>
</template>

<style scoped>
.overlay-container {
  width: 100%;
  min-height: 100%;
  background: rgba(30, 30, 30, 0.95);
  border-radius: 8px;
  overflow: hidden;
}

.overlay-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: rgba(40, 40, 40, 0.98);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  user-select: none;
  cursor: default;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  gap: 8px;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  border-radius: 6px;
  color: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  transition: all 0.15s ease;
}

.toolbar-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

.toolbar-btn.active {
  background: rgba(59, 130, 246, 0.5);
  color: white;
}

.overlay-content {
  padding: 8px;
  min-height: 200px;
}
</style>
