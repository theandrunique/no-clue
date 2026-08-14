import { audioApi } from "$lib/api/audio";
import { createQuery } from "@tanstack/svelte-query";

export function useInputDevices() {
  return createQuery(() => ({
    queryKey: ["audio", "input"],
    queryFn: audioApi.getInputDevices
  }));
}

export function useOutputDevices() {
  return createQuery(() => ({
    queryKey: ["audio", "output"],
    queryFn: audioApi.getOutputDevices
  }));
}
