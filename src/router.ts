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
  // {
  //   path: "/add",
  //   name: "add",
  //   component: () => import("./components/AddTutorial.vue"),
  // },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;