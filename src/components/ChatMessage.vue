<script setup lang="ts">
import { computed, ref, watch, watchEffect } from "vue";
import { marked } from "marked";
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir, join } from "@tauri-apps/api/path";
import { Message } from "@/types";

interface Props {
  message: Message;
  isStreaming?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isStreaming: false,
});

const renderedContent = ref("");

const finalRenderedContent = computed(() => {
  return marked.parse(props.message.content, { async: false }) as string;
});

watch(
  () => props.message.content,
  (newContent) => {
    renderedContent.value = marked.parse(newContent, { async: false }) as string;
  },
  { immediate: true },
);

const screenshotSrc = ref<string | null>(null);

watchEffect(async () => {
  if (props.message.screenshotPath && !props.isStreaming) {
    const dataDir = await appDataDir();
    const absolutePath = await join(dataDir, props.message.screenshotPath);
    screenshotSrc.value = convertFileSrc(absolutePath);
  } else {
    screenshotSrc.value = null;
  }
});
</script>

<template>
  <div
    class="max-w-[85%] px-3 py-2 rounded-lg text-sm wrap-break-words"
    :class="message.role === 'user' ? 'self-end bg-primary/80' : 'self-start bg-muted/80'"
  >
    <img
      v-if="screenshotSrc"
      :src="screenshotSrc"
      alt="Screenshot"
      class="mb-2 rounded-md max-w-full cursor-pointer hover:opacity-90"
    />
    <div
      class="markdown-content"
      :class="{ 'typing-animation': isStreaming }"
      v-html="isStreaming ? renderedContent : finalRenderedContent"
    />
  </div>
</template>

<style scoped>
.typing-animation {
  overflow: hidden;
  animation: typing-fade 0.3s ease-out forwards;
}

@keyframes typing-fade {
  from {
    opacity: 0.7;
  }
  to {
    opacity: 1;
  }
}

.markdown-content :deep(pre) {
  background-color: rgb(0 0 0 / 0.3);
  border-radius: 0.375rem;
  padding: 0.5rem;
  overflow-x: auto;
  margin: 0.25rem 0;
}

.markdown-content :deep(code) {
  background-color: rgb(0 0 0 / 0.3);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
}

.markdown-content :deep(pre code) {
  background: none;
  padding: 0;
}

.markdown-content :deep(ul),
.markdown-content :deep(ol) {
  margin-left: 1rem;
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.markdown-content :deep(li) {
  margin-top: 0.125rem;
}

.markdown-content :deep(p) {
  margin: 0.25rem 0;
}

.markdown-content :deep(p:first-child) {
  margin-top: 0;
}

.markdown-content :deep(p:last-child) {
  margin-bottom: 0;
}

.markdown-content :deep(h1),
.markdown-content :deep(h2),
.markdown-content :deep(h3) {
  font-weight: 600;
  margin-top: 0.5rem;
  margin-bottom: 0.25rem;
}

.markdown-content :deep(h1:first-child),
.markdown-content :deep(h2:first-child),
.markdown-content :deep(h3:first-child) {
  margin-top: 0;
}
</style>