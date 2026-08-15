const KEY = "active_system_prompt_id";

function createActivePromptStore() {
  let active = $state(localStorage.getItem(KEY));

  function toggleActivePrompt(id: string) {
    if (active === id) {
      active = null;
      localStorage.removeItem(KEY);
    } else {
      active = id;
      localStorage.setItem(KEY, id);
    }
  }

  return {
    get activePromptId() {
      return active;
    },
    toggleActivePrompt
  };
}

export const activePromptStore = createActivePromptStore();
