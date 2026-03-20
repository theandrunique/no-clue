import { type Component } from "vue";
import { MessageSquare, Settings } from "lucide-vue-next";

export interface Tab {
  id: string;
  label: string;
  path: string;
  icon: Component;
}

export const tabs: Tab[] = [
  { id: "conversations", label: "Conversations", path: "/dashboard/conversations", icon: MessageSquare },
  { id: "settings", label: "Settings", path: "/dashboard/settings", icon: Settings },
];

export function getTabByPath(path: string): Tab | undefined {
  return tabs.find((tab) => tab.path === path);
}

export function getTabById(id: string): Tab | undefined {
  return tabs.find((tab) => tab.id === id);
}
