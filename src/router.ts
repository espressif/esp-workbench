import { createRouter, createWebHistory } from 'vue-router';
import { RouteRecordRaw } from "vue-router";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    alias: "/dashboard",
    name: "tutorials",
    component: () => import("./components/Dashboard.vue"),
  },
  {
    path: "/esp-idf",
    name: "ESP-IDF",
    component: () => import("./components/EspIdf.vue"),
  },
  {
    path: "/monitor/:portName",
    name: "ESP Monitor",
    component: () => import("./components/Monitor.vue"),
  },
  {
    path: "/flash/:portName",
    name: "flash",
    component: () => import("./components/Flash.vue"),
  },
  {
    path: "/rust",
    name: "rust",
    component: () => import("./components/RustDetail.vue"),
  },
  {
    path: "/rust/book",
    name: "RustBook",
    component: () => import("./components/RustBook.vue"),
  },
  {
    path: "/developer-portal/read",
    name: "DeveloperPortalRead",
    component: () => import("./components/DeveloperPortalRead.vue"),
  },
  {
    path: "/developer-portal/contribute",
    name: "DeveloperPortalContribute",
    component: () => import("./components/DeveloperPortalContribute.vue"),
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
