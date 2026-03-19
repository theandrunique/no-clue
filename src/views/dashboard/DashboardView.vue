<script setup lang="ts">
import { MessageSquare, Settings } from "lucide-vue-next";
import { useRoute } from "vue-router";
import ConversationsTab from "./components/ConversationsTab.vue";

const route = useRoute();
</script>

<template>
  <div class="dashboard h-full flex flex-col">
    <nav class="flex items-center justify-between px-4 py-2 border-b border-neutral-800">
      <div class="flex gap-4">
        <router-link
          to="/dashboard/conversations"
          class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm transition-colors"
          :class="route.path === '/dashboard/conversations' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:text-white'"
        >
          <MessageSquare class="w-4 h-4" />
          Conversations
        </router-link>
        <router-link
          to="/dashboard/settings"
          class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm transition-colors"
          :class="route.path === '/dashboard/settings' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:text-white'"
        >
          <Settings class="w-4 h-4" />
          Settings
        </router-link>
      </div>
    </nav>

    <div class="flex-1 min-h-0">
      <ConversationsTab v-if="route.path === '/dashboard/conversations'" />
      <div v-else-if="route.path === '/dashboard/settings'" class="p-4 text-neutral-400">
        Settings - to be implemented
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  background: #1a1a1a;
  color: white;
}
</style>
