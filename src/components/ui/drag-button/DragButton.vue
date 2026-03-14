<script setup lang="ts">
import { GripVertical } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
</script>

<template>
  <Button
    variant="ghost"
    size="icon"
    class="-ml-[2px] w-fit"
    data-tauri-drag-region
  >
    <GripVertical :size="16" />
  </Button>
</template>
