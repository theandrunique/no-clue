<script setup lang="ts">
import Popover from '@/components/ui/popover/Popover.vue';
import PopoverContent from '@/components/ui/popover/PopoverContent.vue';
import PopoverTrigger from '@/components/ui/popover/PopoverTrigger.vue';
import { Button } from '@/components/ui/button';
import { ChevronDown, ChevronUp, MessageSquare, FileText } from 'lucide-vue-next';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useOverlayStore } from '@/stores/overlay';
import ChatTab from './ChatTab.vue';
import TranscriptTab from './TranscriptTab.vue';

const overlayStore = useOverlayStore();

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
      align="end"
      side="bottom"
      :sideOffset="10"
      class="dark w-screen h-[246px] p-0"
      @interactOutside="handleInteractOutside"
    >
        <Tabs defaultValue="chat" class="h-full">
          <TabsList class="w-full justify-start rounded-none border-b bg-transparent px-2">
            <TabsTrigger value="chat" class="gap-2">
              <MessageSquare class="w-4 h-4" />
              Chat
            </TabsTrigger>
            <TabsTrigger value="transcript" class="gap-2">
              <FileText class="w-4 h-4" />
              Transcript
            </TabsTrigger>
          </TabsList>

          <TabsContent value="chat" class="h-full">
            <ChatTab />
          </TabsContent>
          <TabsContent value="transcript">
            <TranscriptTab />
          </TabsContent>
        </Tabs>
    </PopoverContent>
</Popover>
</template>
