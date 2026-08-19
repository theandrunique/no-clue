<script lang="ts">
  import { Button, Dialog } from "$lib/components/ui";
  import { superForm, defaults } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import { useCreateSystemPrompt } from "$lib/queries/systemPrompts";
  import TextField from "$lib/components/form/TextField.svelte";
  import { createSystemPromptSchema } from "./systemPromptSchema";
  import { createSystemPromptModal } from "$services/system-prompts/createSystemPromptModal.svelte";

  let mutation = useCreateSystemPrompt();

  const { reset, enhance, errors, form, constraints } = superForm(defaults(zod4(createSystemPromptSchema)), {
    SPA: true,
    validators: zod4(createSystemPromptSchema),
    onUpdate({ form: v }) {
      if (!v.valid) return;

      mutation.mutate(v.data, {
        onSuccess: () => createSystemPromptModal.close()
      });
    },
    resetForm: false
  });

  $effect.pre(() => {
    if (createSystemPromptModal.isOpen) {
      reset();
    }
  });

  function handleOpenChange(newOpen: boolean) {
    if (newOpen === true) return;
    createSystemPromptModal.close();
  }
</script>

<Dialog.Root bind:open={() => createSystemPromptModal.isOpen, handleOpenChange}>
  <Dialog.Portal>
    <Dialog.Overlay />

    <Dialog.Content>
      <Dialog.Title>Create new system prompt</Dialog.Title>

      <Dialog.Close />

      <form class="mt-4 flex flex-col gap-4" use:enhance novalidate>
        <TextField
          label="Name"
          placeholder="Enter the name"
          bind:value={$form.name}
          errors={$errors.name}
          {...$constraints.name}
        />

        <TextField
          label="Prompt"
          placeholder="Enter the prompt"
          bind:value={$form.prompt}
          errors={$errors.prompt}
          {...$constraints.prompt}
        />

        <div class="flex items-center justify-end gap-2">
          <Button type="button" variant="secondary" onclick={() => handleOpenChange(false)}>Cancel</Button>

          <Button disabled={mutation.isPending}>
            {mutation.isPending ? "Saving..." : "Create"}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
