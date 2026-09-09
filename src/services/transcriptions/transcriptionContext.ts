import type { TranscriptionStatus, TranscriptResult } from "$lib/types";
import { createContext } from "svelte";

export interface TranscriptionContext {
  readonly status: TranscriptionStatus;
  readonly interim: TranscriptResult[];
  toggle: () => void;
}

export const [getTranscriptionContext, setTranscriptionContext] = createContext<TranscriptionContext>();
