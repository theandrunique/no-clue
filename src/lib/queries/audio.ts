import { audioIpc } from "$lib/ipc/audio";
import { createQuery } from "@tanstack/svelte-query";

export function useInputDevices() {
  return createQuery(() => ({
    queryKey: ["audio", "input"],
    queryFn: audioIpc.getInputDevices
  }));
}

export function useOutputDevices() {
  return createQuery(() => ({
    queryKey: ["audio", "output"],
    queryFn: audioIpc.getOutputDevices
  }));
}
