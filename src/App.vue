<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import { appWindow } from '@tauri-apps/api/window';
import HomeIcon from "./components/HomeIcon.vue";
import ErrorMessage from './components/ErrorMessage.vue';
import Console from './components/Console.vue';

const appVersion = ref('');
const errorMessage = ref("");

async function fetchVersion() {
  appVersion.value = await getVersion();
}

type Payload = {
  pct: string,
}

onMounted(() => {
  appWindow.listen('error', (event) => {
    const payload = event.payload as Payload;
    if (payload.pct.startsWith('Error: ')) {
      errorMessage.value = payload.pct.substring('Error: '.length);
    }
  });

  fetchVersion();
});
</script>

<template>
  <div class="container">
    <aside class="sidebar">
      <router-link to="/" class="nav-icon">
        <HomeIcon />
      </router-link>
    </aside>
    <main>
      <ErrorMessage v-if="errorMessage" :message="errorMessage" @dismiss="errorMessage = ''" />
      <h1>ESP Workbench</h1>
      <div>Navigate with ease in the world of ESP32</div>
      <div class="router-view">
        <router-view />
      </div>
      <div class="version">Version: {{ appVersion }}</div>
    </main>
    <Console />
  </div>
</template>

<style scoped>
.router-view {
  height: 78vh;
}
.version {
  position: fixed;
  bottom: 10px;
  right: 10px;
  font-size: xx-small;
}

.container {
  display: flex;;
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
