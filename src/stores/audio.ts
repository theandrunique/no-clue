import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}

const STORAGE_KEY = "no-clue-audio-settings";

export interface AudioSettings {
  system_device_id: string | null;
  microphone_device_id: string | null;
  capture_system: boolean;
  capture_microphone: boolean;
}

const defaultSettings: AudioSettings = {
  system_device_id: null,
  microphone_device_id: null,
  capture_system: true,
  capture_microphone: true,
};

function loadSettings(): AudioSettings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaultSettings, ...JSON.parse(stored) };
    }
  } catch {
    // ignore
  }
  return defaultSettings;
}

export const useAudioStore = defineStore("audio", () => {
  const inputDevices = ref<AudioDevice[]>([]);
  const outputDevices = ref<AudioDevice[]>([]);
  const loading = ref(false);
  const settings = ref<AudioSettings>(loadSettings());

  const captureSystem = computed(() => settings.value.capture_system);
  const captureMicrophone = computed(() => settings.value.capture_microphone);

  function saveSettings() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value));
  }

  function setSystemDevice(deviceId: string | null) {
    const defaultDevice = outputDevices.value.find(d => d.is_default);
    if (deviceId === defaultDevice?.id) {
      settings.value.system_device_id = null;
    } else {
      settings.value.system_device_id = deviceId;
    }
    saveSettings();
  }

  function setMicrophoneDevice(deviceId: string | null) {
    const defaultDevice = inputDevices.value.find(d => d.is_default);
    if (deviceId === defaultDevice?.id) {
      settings.value.microphone_device_id = null;
    } else {
      settings.value.microphone_device_id = deviceId;
    }
    saveSettings();
  }

  function setCaptureSystem(value: boolean) {
    settings.value.capture_system = value;
    saveSettings();
  }

  function setCaptureMicrophone(value: boolean) {
    settings.value.capture_microphone = value;
    saveSettings();
  }

  async function loadDevices() {
    loading.value = true;
    try {
      const [inputs, outputs] = await Promise.all([
        invoke<AudioDevice[]>("get_input_devices"),
        invoke<AudioDevice[]>("get_output_devices"),
      ]);
      inputDevices.value = inputs;
      outputDevices.value = outputs;
    } catch (e) {
      console.error("[AudioStore] Failed to load devices:", e);
    } finally {
      loading.value = false;
    }
  }

  function getSystemDeviceId(): string | null {
    if (settings.value.system_device_id !== null) {
      return settings.value.system_device_id;
    }
    const defaultDevice = outputDevices.value.find(d => d.is_default);
    return defaultDevice?.id || null;
  }

  function getMicrophoneDeviceId(): string | null {
    if (settings.value.microphone_device_id !== null) {
      return settings.value.microphone_device_id;
    }
    const defaultDevice = inputDevices.value.find(d => d.is_default);
    return defaultDevice?.id || null;
  }

  return {
    inputDevices,
    outputDevices,
    loading,
    settings,
    captureSystem,
    captureMicrophone,
    loadDevices,
    setSystemDevice,
    setMicrophoneDevice,
    setCaptureSystem,
    setCaptureMicrophone,
    getSystemDeviceId,
    getMicrophoneDeviceId,
  };
});
