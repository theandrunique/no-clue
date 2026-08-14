const STORAGE_KEY = "no-clue-audio-settings";

export interface AudioSettings {
  system_device_id: string | null;
  microphone_device_id: string | null;
  capture_system: boolean;
  capture_microphone: boolean;
}

const defaultAudioSettings: AudioSettings = {
  system_device_id: null,
  microphone_device_id: null,
  capture_system: true,
  capture_microphone: true
};

function loadAudioSettings(): AudioSettings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaultAudioSettings, ...JSON.parse(stored) };
    }
    // eslint-disable-next-line no-empty
  } catch {}

  return defaultAudioSettings;
}

function createAudioStore() {
  const settings = $state(loadAudioSettings());
  save();

  function save() {
    console.log("Saving");
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  }

  return {
    get system_device_id() {
      return settings.system_device_id;
    },
    set system_device_id(value) {
      settings.system_device_id = value;
      save();
    },
    get microphone_device_id() {
      return settings.microphone_device_id;
    },
    set microphone_device_id(value) {
      settings.microphone_device_id = value;
      save();
    },
    get capture_system() {
      return settings.capture_system;
    },
    set capture_system(value) {
      settings.capture_system = value;
      save();
    },
    get capture_microphone() {
      return settings.capture_microphone;
    },
    set capture_microphone(value) {
      settings.capture_microphone = value;
      save();
    }
  };
}

export const audioSettingsStore = createAudioStore();
