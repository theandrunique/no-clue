import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "overlay",
      component: () => import("../views/overlay/OverlayView.vue"),
    },
    {
      path: "/dashboard/conversations",
      name: "conversations",
      component: () => import("../views/dashboard/DashboardView.vue"),
    },
    {
      path: "/dashboard/settings",
      name: "settings",
      component: () => import("../views/dashboard/DashboardView.vue"),
    },
  ],
});

export default router;
