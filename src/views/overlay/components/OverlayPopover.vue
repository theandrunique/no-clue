<script setup lang="ts">
import { computed } from 'vue';
import Popover from '@/components/ui/popover/Popover.vue';
import PopoverContent from '@/components/ui/popover/PopoverContent.vue';
import PopoverTrigger from '@/components/ui/popover/PopoverTrigger.vue';
import { Button } from '@/components/ui/button';
import { ChevronDown, ChevronUp, MessageSquare, FileText } from 'lucide-vue-next';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useOverlayStore } from '@/stores/overlay';
import { useSettingsStore } from '@/stores/settings';
import ChatTab from './ChatTab.vue';
import TranscriptTab from './TranscriptTab.vue';

const overlayStore = useOverlayStore();
const settingsStore = useSettingsStore();

const popoverStyle = computed(() => ({
  '--overlay-opacity': settingsStore.settings.overlayOpacity,
  width: `${settingsStore.settings.overlayWidth}px`,
  height: `${settingsStore.settings.overlayHeight - 59}px`,
}));

function handleOpenChange(open: boolean) {
  if (open !== overlayStore.isExpanded) {
    overlayStore.setExpanded(open);
  }
}

function handleInteractOutside(event: Event) {
  event.preventDefault();
}
</script>

<template>
<Popover :open="overlayStore.isExpanded" @update:open="handleOpenChange">
    <PopoverTrigger asChild>
        <Button variant="ghost" size="icon">
            <ChevronDown v-if="!overlayStore.isExpanded" class="w-4 h-4" />
            <ChevronUp v-else class="w-4 h-4" />
        </Button>
    </PopoverTrigger>
    <PopoverContent
      :style="popoverStyle"
      align="end"
      side="bottom"
      :sideOffset="10"
      class="dark p-0 overflow-hidden overlay-card"
      @interactOutside="handleInteractOutside"
    >
        <Tabs defaultValue="chat" class="h-full flex flex-col">
          <TabsList class="shrink-0 w-full justify-start rounded-none border-b bg-transparent px-2 h-9">
            <TabsTrigger value="chat" class="gap-2">
              <MessageSquare class="w-4 h-4" />
              Chat
            </TabsTrigger>
            <TabsTrigger value="transcript" class="gap-2">
              <FileText class="w-4 h-4" />
              Transcript
            </TabsTrigger>
          </TabsList>

          <TabsContent value="chat" class="flex-1 min-h-0 m-0 overflow-hidden">
            <ChatTab />
          </TabsContent>
          <TabsContent value="transcript" class="flex-1 min-h-0 m-0 overflow-hidden">
            <TranscriptTab />
          </TabsContent>
        </Tabs>
    </PopoverContent>
</Popover>
</template>
