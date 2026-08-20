import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { ChatStreamEvent, TranscriptResult } from "$lib/types";

export const Events = {
  chatStream: "chat-stream",
  transcriptionResult: "transcription-result",
  transcriptionStopping: "transcription-stopping",
  transcriptionStopped: "transcription-stopped",
  shortcutTriggered: "shortcut-triggered",
  testStreamError: "test-stream-error",
  testStreamStarted: "test-stream-started",
  testStreamProgress: "test-stream-progress",
  testStreamStopped: "test-stream-stopped",
  testMicError: "test-mic-error",
  testMicStarted: "test-mic-started",
  testMicProgress: "test-mic-progress",
  testMicStopped: "test-mic-stopped"
} as const;

export interface EventMap {
  [Events.chatStream]: ChatStreamEvent;
  [Events.transcriptionResult]: TranscriptResult;
  [Events.transcriptionStopping]: void;
  [Events.transcriptionStopped]: void;
  [Events.shortcutTriggered]: string;
  [Events.testStreamError]: string;
  [Events.testStreamStarted]: number;
  [Events.testStreamProgress]: [number, number];
  [Events.testStreamStopped]: [number, number];
  [Events.testMicError]: string;
  [Events.testMicStarted]: number;
  [Events.testMicProgress]: [number, number];
  [Events.testMicStopped]: [number, number];
}

export function listenEvent<K extends keyof EventMap>(
  event: K,
  handler: (payload: EventMap[K]) => void
): Promise<UnlistenFn> {
  return listen<EventMap[K]>(event, (e) => handler(e.payload));
}
