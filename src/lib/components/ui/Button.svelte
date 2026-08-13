<script lang="ts">
  import { cn } from "$lib/utils";
  import { Button } from "bits-ui";
  import { cva, type VariantProps } from "class-variance-authority";

  const buttonVariants = cva("", {
    variants: {
      variant: {
        primary:
          "bg-(--button-bg-primary) hover:bg-(--button-bg-primary-hover) active:bg-(--button-bg-primary-active) text-(--text-on-primary)",
        accent:
          "bg-(--button-bg-accent) hover:bg-(--button-bg-accent-hover) active:bg-(--button-bg-accent-active)",
        secondary:
          "bg-(--button-bg-secondary) hover:bg-(--button-bg-secondary-hover) active:bg-(--button-bg-secondary-active)",
        ghost:
          "bg-transparent hover:bg-(--button-bg-secondary-hover) active:bg-(--button-bg-secondary-active)",
        icon: "bg-transparent hover:bg-(--button-bg-secondary-hover) active:bg-(--button-bg-secondary-active) p-(--button-padding-y)"
      }
    },
    defaultVariants: {
      variant: "primary"
    }
  });

  type Variant = VariantProps<typeof buttonVariants>["variant"];
  type Props = Button.RootProps & {
    variant?: Variant;
  };

  let { variant = "primary", class: className = "", children, ...rest }: Props = $props();

  let classes = $derived(
    cn(
      "px-(--button-padding-x) py-(--button-padding-y) rounded-(--button-radius) select-none",
      "text-base font-semibold text-(--color-text)",
      "flex gap-2 items-center",
      "disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed",
      buttonVariants({ variant }),
      className
    )
  );
</script>

<Button.Root class={classes} {...rest}>
  {@render children?.()}
</Button.Root>
