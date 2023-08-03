<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import Dashboard from "./components/Dashboard.vue";
import HomeIcon from "./components/HomeIcon.vue";
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
    <aside class="sidebar">
      <router-link to="/" class="nav-icon">
        <HomeIcon />
      </router-link>
      <!-- Add more navigation links/icons here if needed -->
    </aside>
    <main>
      <h1>ESP Helm</h1>
      <div>Navigate with ease in the world of ESP32</div>
      <router-view />
      <div class="version">Version: {{ appVersion }}</div>
    </main>
  </div>
</template>

<style scoped>
.version {
  position: fixed;
  bottom: 10px;
  right: 10px;
  font-size: xx-small;
}

.container {
  display: flex;
}

.sidebar {
  position: fixed;
  top: 0;
  bottom: 0;
  flex-direction: column;
  align-items: center;
  /* background-color: #555; */
  /* color: #fff; */
  width: 60px;
  padding: 10px 10px;
}

.nav-icon {
  margin-bottom: 20px;
}

.icon {
  width: 30px;
  height: 30px;
}

main {
  flex: 1;
  padding-left: 20px;
}
</style>
