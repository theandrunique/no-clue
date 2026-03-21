import { type Component } from "vue";
import { MessageSquare, Settings, Terminal, Keyboard, Volume2, Cpu } from "lucide-vue-next";

export interface Tab {
  id: string;
  label: string;
  path: string;
  icon: Component;
}

export const tabs: Tab[] = [
  { id: "conversations", label: "Conversations", path: "/dashboard/conversations", icon: MessageSquare },
  { id: "system-prompts", label: "System Prompts", path: "/dashboard/system-prompts", icon: Terminal },
  { id: "shortcuts", label: "Shortcuts", path: "/dashboard/shortcuts", icon: Keyboard },
  { id: "audio", label: "Audio", path: "/dashboard/audio", icon: Volume2 },
  { id: "providers", label: "Providers", path: "/dashboard/providers", icon: Cpu },
  { id: "settings", label: "Settings", path: "/dashboard/settings", icon: Settings },
];

export function getTabByPath(path: string): Tab | undefined {
  return tabs.find((tab) => tab.path === path);
}

export function getTabById(id: string): Tab | undefined {
  return tabs.find((tab) => tab.id === id);
}
