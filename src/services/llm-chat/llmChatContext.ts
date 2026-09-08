import type { Message } from "$lib/types";
import { createContext } from "svelte";

export interface LlmChatContext {
  readonly isStreaming: boolean;
  readonly currentMessage: Message | null;
}

export const [getLlmChatContext, setLlmChatContext] = createContext<LlmChatContext>();
