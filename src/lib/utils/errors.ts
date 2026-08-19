export interface TauriError {
  type?: string;
  message?: string;
}

const ERROR_MESSAGES: Record<string, string> = {
  Internal: "Internal error",
  Database: "Database error",
  SystemPromptNotFound: "System prompt not found",
  ConversationNotFound: "Conversation not found",
  MessageNotFound: "Message not found",
  LlmProviderNotConfigured: "LLM provider is not configured",
  SttProviderNotConfigured: "STT provider is not configured",
  LlmProviderAlreadyRunning: "LLM provider is already running",
  SttProviderAlreadyRunning: "STT provider is already running",
  AtLeactOneAudioSourceMustBeEnabled: "At least one audio source must be enabled",
  TranscriptionConversationIdNotSet: "Transcription conversation is not set",
  OverlayAlreadyRunning: "Overlay is already running",
  OverlayNotRunning: "Overlay is not running"
};

export function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;

  if (typeof error === "object" && error !== null) {
    const tauriError = error as TauriError;
    if (tauriError.type) {
      return ERROR_MESSAGES[tauriError.type] ?? tauriError.message ?? "Unknown error";
    }
    if (tauriError.message) return tauriError.message;
  }

  if (typeof error === "string") return error;

  return "Unknown error";
}
