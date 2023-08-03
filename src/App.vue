<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import Dashboard from "./components/Dashboard.vue";

const appVersion = ref('');

async function fetchVersion() {
  appVersion.value = await getVersion();
}

onMounted(() => {
  fetchVersion();
});

</script>

<template>
  <div class="container">
    <h1>ESP Helm</h1>
    <div>Navigate with ease in the world of ESP32</div>
    <!-- <Dashboard /> -->
      <router-view />
    <div class="version">Version: {{ appVersion }}</div>
  </div>
</template>

<style scoped>
.version {
  position: fixed;
  bottom: 10px;
  right: 10px;
  font-size: xx-small;
}
</style>
