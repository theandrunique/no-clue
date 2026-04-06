import { createRouter, createWebHistory } from "vue-router";
import type { RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "overlay",
    component: () => import("../views/overlay/OverlayView.vue"),
  },
  {
    path: "/dashboard",
    component: () => import("../views/dashboard/DashboardLayout.vue"),
    children: [
      {
        path: "",
        redirect: "/dashboard/conversations",
      },
      {
        path: "conversations",
        name: "conversations",
        component: () => import("../views/dashboard/tabs/ConversationsTab"),
      },
      {
        path: "system-prompts",
        name: "system-prompts",
        component: () => import("../views/dashboard/tabs/SystemPromptsTab"),
      },
      {
        path: "shortcuts",
        name: "shortcuts",
        component: () => import("../views/dashboard/tabs/ShortcutsTab.vue"),
      },
      {
        path: "audio",
        name: "audio",
        component: () => import("../views/dashboard/tabs/AudioTab.vue"),
      },
      {
        path: "providers",
        name: "providers",
        component: () => import("../views/dashboard/tabs/ProvidersTab.vue"),
      },
      {
        path: "settings",
        name: "settings",
        component: () => import("../views/dashboard/tabs/SettingsTab.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
