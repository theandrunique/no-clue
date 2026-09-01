import type { LlmProvider } from "$lib/types/providers";

function createLlmProviderConnectModal() {
  let status = $state<{
    provider: LlmProvider | null;
    isOpen: boolean;
  }>({
    provider: null,
    isOpen: false
  });

  function open(provider: LlmProvider) {
    status = {
      provider,
      isOpen: true
    };
  }

  function close() {
    status = { ...status, isOpen: false };
  }

  return {
    get status() {
      return status;
    },
    open,
    close
  };
}

export const llmProviderConnectModal = createLlmProviderConnectModal();
