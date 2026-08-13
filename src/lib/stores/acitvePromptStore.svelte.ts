const KEY = "active_system_prompt_id";

function createActivePromptStore() {
  let active = $state(localStorage.getItem(KEY));

  function setActivePrompt(id: string) {
    localStorage.setItem(KEY, id);
    active = id;
  }

  return {
    get activePromptId() {
      return active;
    },
    setActivePrompt
  };
}

export const activePromptStore = createActivePromptStore();
