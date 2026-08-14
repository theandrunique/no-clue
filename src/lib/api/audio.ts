import type { AudioDevice } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const audioApi = {
  getInputDevices: () => invoke<AudioDevice[]>("get_input_devices"),
  getOutputDevices: () => invoke<AudioDevice[]>("get_output_devices")
};
