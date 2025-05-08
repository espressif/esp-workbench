<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import { appWindow } from '@tauri-apps/api/window';
import HomeIcon from "./icons/HomeIcon.vue";
import ConsoleIcon from './icons/ConsoleIcon.vue';
import SettingsIcon from './icons/SettingsIcon.vue';
import Console from "./components/Console.vue";
import ErrorMessage from './components/ErrorMessage.vue';

const appVersion = ref('');
const errorMessage = ref("");
const isConsoleVisible = ref(false);

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

const toggleConsole = () => {
  isConsoleVisible.value = !isConsoleVisible.value;
};
</script>

<template>
  <div class="container">
    <aside class="sidebar">
      <router-link to="/" class="nav-icon">
        <HomeIcon />
      </router-link>
      <router-link to="/settings" class="nav-icon">
        <SettingsIcon />
      </router-link>
      <button class="toggle-button" @click="toggleConsole">
        <ConsoleIcon />
      </button>
      <div class="version">Version: {{ appVersion }}</div>
    </aside>
    <main>
      <ErrorMessage v-if="errorMessage" :message="errorMessage" @dismiss="errorMessage = ''" />
      <h1>ESP Workbench</h1>
      <div>Navigate with ease in the world of ESP32</div>
      <div class="router-view">
        <router-view />
      </div>
    </main>
    <Console :is-visible="isConsoleVisible" />
  </div>
</template>

<style scoped src="./App.css"></style>
