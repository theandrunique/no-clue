<script setup lang="ts">
import { ref, watch } from "vue";
import { marked } from "marked";

interface Props {
  content: string;
}

const props = defineProps<Props>();

const renderedContent = ref("");

watch(
  () => props.content,
  (newContent) => {
    renderedContent.value = marked.parse(newContent, { async: false }) as string;
  },
  { immediate: true },
);
</script>

<template>
  <div class="self-start bg-muted/80 px-3 py-2 rounded-lg text-sm">
    <div class="markdown-content" v-html="renderedContent" /><span class="animate-pulse">▊</span>
  </div>
</template>

<style scoped>
.markdown-content {
  display: inline;
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
