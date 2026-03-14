<script setup lang="ts">
import type { PopoverTriggerProps } from "reka-ui"
import { PopoverTrigger as PopoverTriggerPrimitive } from "reka-ui"
import { cn } from "@/lib/utils"

const props = defineProps<PopoverTriggerProps & { class?: string }>()
</script>

<template>
  <PopoverTriggerPrimitive
    v-bind="props"
    :class="cn(
      'data-[state=open]:bg-primary-foreground data-[state=open]:text-primary data-[state=open]:border-primary/20 data-[state=open]:border-1 transition-all duration-300',
      props.class,
    )"
  >
    <slot />
  </PopoverTriggerPrimitive>
</template>
