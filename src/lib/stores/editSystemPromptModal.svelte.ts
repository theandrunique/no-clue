import type { SystemPrompt } from "$lib/types";

function createEditSystemPromptsModal() {
  let status = $state<{
    systemPrompt: SystemPrompt | null;
    isOpen: boolean;
  }>({
    systemPrompt: null,
    isOpen: false
  });

  function open(systemPrompt: SystemPrompt) {
    status = {
      systemPrompt,
      isOpen: true
    };
  }

  function close() {
    status = {
      ...status,
      isOpen: false
    };
  }

  return {
    get status() {
      return status;
    },
    open,
    close
  };
}

export const editSystemPromptModal = createEditSystemPromptsModal();
