<script lang="ts">
  import { Button, Dialog } from "$lib/components/ui";
  import { superForm, defaults } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import { useUpdateSystemPrompt } from "$lib/queries/systemPrompts";
  import TextField from "$lib/components/form/TextField.svelte";
  import { createSystemPromptSchema } from "$lib/schemas/systemPrompts";
  import { editSystemPromptModal } from "$lib/stores/editSystemPromptModal.svelte";

  let mutation = useUpdateSystemPrompt();

  const { enhance, errors, form, constraints, reset } = superForm(
    defaults(zod4(createSystemPromptSchema)),
    {
      SPA: true,
      validators: zod4(createSystemPromptSchema),
      onUpdate({ form: v }) {
        if (!v.valid) return;
        if (!editSystemPromptModal.status.systemPrompt) return;

        mutation.mutate(
          { ...v.data, id: editSystemPromptModal.status.systemPrompt.id },
          {
            onSuccess: () => editSystemPromptModal.close()
          }
        );
      },
      resetForm: false
    }
  );

  $effect.pre(() => {
    if (editSystemPromptModal.status.isOpen) {
      reset({
        data: {
          ...editSystemPromptModal.status.systemPrompt
        }
      });
    }
  });

  function handleOpenChange(newOpen: boolean) {
    if (newOpen === true) return;
    editSystemPromptModal.close();
  }
</script>

<Dialog.Root bind:open={() => editSystemPromptModal.status.isOpen, handleOpenChange}>
  <Dialog.Portal>
    <Dialog.Overlay />

    <Dialog.Content>
      <Dialog.Title>Update system prompt</Dialog.Title>

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
          <Button type="button" variant="secondary" onclick={() => handleOpenChange(false)}>
            Cancel
          </Button>

          <Button disabled={mutation.isPending}>
            {mutation.isPending ? "Saving..." : "Update"}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
