<script lang="ts">
  import { page } from "$app/state";
  import { Bot, Logs, MessageSquare, Play, Settings } from "@lucide/svelte";
  import type { Component } from "svelte";
  import { cn } from "$lib/utils";
  import { Button } from "$lib/components/ui";
  import { overlaySessionService } from "$services/overlay/overlaySession.svelte";

  interface NavItem {
    href: string;
    label: string;
    icon: Component;
  }

  const navigation: NavItem[] = [
    { href: "/overview", label: "Overview", icon: Logs },
    { href: "/conversations", label: "Conversations", icon: MessageSquare },
    { href: "/system-prompts", label: "System Prompts", icon: Bot },
    { href: "/settings", label: "Settings", icon: Settings }
  ];

  function isActive(item: NavItem) {
    return page.url.pathname.startsWith(item.href);
  }
</script>

<div class="flex flex-col gap-3 border-r border-(--color-border) bg-(--bg-card) p-2">
  <Button onclick={() => overlaySessionService.startNew()}>
    <Play />
    Start
  </Button>

  <nav class="flex min-w-0 flex-1 flex-col gap-2 overflow-y-auto">
    {#each navigation as item (item.href)}
      <Button variant="ghost" href={item.href} class={cn(isActive(item) ? "bg-(--button-bg-secondary-hover)" : "")}>
        <item.icon />
        {item.label}
      </Button>
    {/each}
  </nav>
</div>
