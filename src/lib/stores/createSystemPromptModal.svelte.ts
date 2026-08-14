function createCreateSystemPromptsModal() {
  let isOpen = $state(false);

  function open() {
    isOpen = true;
  }

  function close() {
    isOpen = false;
  }

  return {
    get isOpen() {
      return isOpen;
    },
    open,
    close
  };
}

export const createSystemPromptModal = createCreateSystemPromptsModal();
