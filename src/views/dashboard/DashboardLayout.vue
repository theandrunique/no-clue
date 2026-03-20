<script setup lang="ts">
import { useRoute } from "vue-router";
import { tabs } from "@/router/tabs";

const route = useRoute();
</script>

<template>
  <div
    class="dark relative flex flex-col h-screen w-screen overflow-hidden bg-card"
  >
    <nav class="flex w-full justify-center gap-2 py-2 border-b border-border">
      <router-link
        v-for="tab in tabs"
        :key="tab.id"
        :to="tab.path"
        class="inline-flex items-center justify-center gap-2 rounded-xl px-3 py-2 text-sm font-medium text-primary hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 transition-all"
        :class="route.path === tab.path ? 'border-ring/80 ' : ''"
      >
        <component
          :is="tab.icon"
          class="size-4 shrink-0"
        />
        <span>{{ tab.label }}</span>
      </router-link>
    </nav>

    <main class="flex flex-1 flex-col overflow-hidden">
      <RouterView />
    </main>
  </div>
</template>
