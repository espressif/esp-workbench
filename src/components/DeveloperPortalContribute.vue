<template>
  <div>
    <h2>Developer Portal</h2>
    <div v-if="isDevPortalPresent">
      <p>Developer Portal directory is present.</p>
      <button @click="launchHugo">Launch Hugo</button>
      <div v-if="authors.length">
        <h3>Authors</h3>
        <div v-for="author in authors" :key="author.name">
          <h4>{{ author.name }}</h4>
          <img :src="author.image" alt="Author Image" />
          <p>{{ author.bio }}</p>
          <div v-for="social in author.social" :key="Object.keys(social)[0]">
            <a :href="Object.values(social)[0]">{{ Object.keys(social)[0] }}</a>
          </div>
        </div>
      </div>
    </div>
    <div v-else>
      <p>Developer Portal directory is not present.</p>
      <button @click="cloneRepo">Clone Repository</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';

const isDevPortalPresent = ref(false);
const authors = ref([]);

const checkDevPortal = async () => {
  try {
    isDevPortalPresent.value = await invoke('check_devportal');
    if (isDevPortalPresent.value) {
      authors.value = await invoke('get_authors');
    }
  } catch (error) {
    console.error(error);
  }
};

const cloneRepo = async () => {
  try {
    await invoke('execute_command', { command: 'git clone git@github.com:espressif/developer-portal.git ~/.espressif/devportal' });
    checkDevPortal();
  } catch (error) {
    console.error(error);
  }
};

const launchHugo = async () => {
  try {
    await invoke('execute_command', { command: 'hugo server --source ~/.espressif/devportal' });
  } catch (error) {
    console.error(error);
  }
};

onMounted(checkDevPortal);
</script>
