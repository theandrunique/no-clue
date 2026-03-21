<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Conversation, Message, Transcript } from "@/types";
import ConversationList from "./components/ConversationList.vue";
import ConversationDetail from "./components/ConversationDetail.vue";

const conversations = ref<Conversation[]>([]);
const selectedConversation = ref<Conversation | null>(null);
const messages = ref<Message[]>([]);
const transcripts = ref<Transcript[]>([]);
const activeTab = ref<"messages" | "transcripts">("messages");
const loadingMessages = ref(false);
const loadingTranscripts = ref(false);

async function loadConversations() {
  try {
    conversations.value = await invoke<Conversation[]>("get_conversations");
  } catch (e) {
    console.error("Failed to load conversations:", e);
  }
}

async function selectConversation(conversation: Conversation) {
  selectedConversation.value = conversation;
  activeTab.value = "messages";
  messages.value = [];
  transcripts.value = [];

  loadingMessages.value = true;
  try {
    messages.value = await invoke<Message[]>("get_messages", { conversationId: conversation.id });
    console.log("[ConversationsTab] Loaded messages:", messages.value);
  } catch (e) {
    console.error("Failed to load messages:", e);
  } finally {
    loadingMessages.value = false;
  }

  loadingTranscripts.value = true;
  try {
    transcripts.value = await invoke<Transcript[]>("get_transcripts", { conversationId: conversation.id });
    console.log("[ConversationsTab] Loaded transcripts:", transcripts.value);
  } catch (e) {
    console.error("Failed to load transcripts:", e);
  } finally {
    loadingTranscripts.value = false;
  }
}

async function deleteConversation(id: string) {
  try {
    await invoke("delete_conversation", { id });
    conversations.value = conversations.value.filter((c) => c.id !== id);
    if (selectedConversation.value?.id === id) {
      selectedConversation.value = null;
    }
  } catch (e) {
    console.error("Failed to delete conversation:", e);
  }
}

onMounted(() => {
  loadConversations();
});
</script>

<template>
  <div class="flex h-full overflow-hidden">
    <ConversationList
      :conversations="conversations"
      :selected-id="selectedConversation?.id ?? null"
      @select="selectConversation"
      @delete="deleteConversation"
    />

    <ConversationDetail
      v-if="selectedConversation"
      v-model:active-tab="activeTab"
      :conversation="selectedConversation"
      :messages="messages"
      :transcripts="transcripts"
      :loading-messages="loadingMessages"
      :loading-transcripts="loadingTranscripts"
    />

    <div v-else class="flex-1 flex items-center justify-center text-muted-foreground">
      Select a conversation to view details
    </div>
  </div>
</template>