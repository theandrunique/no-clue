<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useAudioStore } from "@/stores/audio";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const audioStore = useAudioStore();

onMounted(async () => {
  await audioStore.loadDevices();
});

const systemDeviceOptions = computed(() => {
  return audioStore.outputDevices;
});

const microphoneDeviceOptions = computed(() => {
  return audioStore.inputDevices;
});

function getSelectedSystemDevice(): string {
  if (audioStore.settings.system_device_id !== null) {
    return audioStore.settings.system_device_id;
  }
  const defaultDevice = audioStore.outputDevices.find(d => d.is_default);
  return defaultDevice?.id || "";
}

function getSelectedMicrophoneDevice(): string {
  if (audioStore.settings.microphone_device_id !== null) {
    return audioStore.settings.microphone_device_id;
  }
  const defaultDevice = audioStore.inputDevices.find(d => d.is_default);
  return defaultDevice?.id || "";
}

function onSystemDeviceChange(value: string | null) {
  audioStore.setSystemDevice(value || null);
}

function onMicrophoneDeviceChange(value: string | null) {
  audioStore.setMicrophoneDevice(value || null);
}
</script>

<template>
  <div class="h-full p-6 overflow-y-auto">
    <div class="max-w-2xl space-y-6">
      <h2 class="text-lg font-medium text-foreground">Audio Settings</h2>

      <Card>
        <CardHeader>
          <CardTitle>System Audio</CardTitle>
          <CardDescription>Audio output from applications</CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center justify-between">
            <label for="capture-system" class="text-sm font-medium">Capture System Audio</label>
            <input
              type="checkbox"
              id="capture-system"
              :checked="audioStore.captureSystem"
              @change="audioStore.setCaptureSystem(($event.target as HTMLInputElement).checked)"
            />
          </div>

          <div class="space-y-2">
            <label for="system-device" class="text-sm font-medium">Output Device</label>
            <Select
              :model-value="getSelectedSystemDevice()"
              @update:model-value="(val: unknown) => onSystemDeviceChange(val as string)"
              :disabled="!audioStore.captureSystem"
            >
              <SelectTrigger id="system-device">
                <SelectValue placeholder="Select device" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem
                  v-for="device in systemDeviceOptions"
                  :key="device.id"
                  :value="device.id"
                >
                  {{ device.name }}<span v-if="device.is_default" class="text-muted-foreground"> (Default)</span>
                </SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Microphone</CardTitle>
          <CardDescription>Audio input from microphone</CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center justify-between">
            <label for="capture-mic" class="text-sm font-medium">Capture Microphone</label>
            <input
              type="checkbox"
              id="capture-mic"
              :checked="audioStore.captureMicrophone"
              @change="audioStore.setCaptureMicrophone(($event.target as HTMLInputElement).checked)"
            />
          </div>

          <div class="space-y-2">
            <label for="mic-device" class="text-sm font-medium">Input Device</label>
            <Select
              :model-value="getSelectedMicrophoneDevice()"
              @update:model-value="(val: unknown) => onMicrophoneDeviceChange(val as string)"
              :disabled="!audioStore.captureMicrophone"
            >
              <SelectTrigger id="mic-device">
                <SelectValue placeholder="Select device" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem
                  v-for="device in microphoneDeviceOptions"
                  :key="device.id"
                  :value="device.id"
                >
                  {{ device.name }}<span v-if="device.is_default" class="text-muted-foreground"> (Default)</span>
                </SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
