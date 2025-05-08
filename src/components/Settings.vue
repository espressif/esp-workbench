<template>
  <div class="settings">
    <h2>Settings</h2>
    <label for="githubUsername">GitHub Username:</label>
    <input v-model="githubUsername" id="githubUsername" placeholder="Enter your GitHub username" />

    <label for="repoPath">Developer Portal Repository Path:</label>
    <input v-model="repoPath" id="repoPath" placeholder="Enter repository path" />

    <button @click="saveSettings">Save</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const githubUsername = ref('');
const repoPath = ref('');

const loadSettings = async () => {
  try {
    const settings = await invoke('load_settings');
    githubUsername.value = settings.github_username || '';
    repoPath.value = settings.developer_portal_repo_path || '~/.espressif/developer-portal';
  } catch (error) {
    console.error(error);
  }
};

const saveSettings = async () => {
  try {
    await invoke('save_settings', { githubUsername: githubUsername.value, developerPortalRepoPath: repoPath.value });
    alert('Settings saved successfully!');
  } catch (error) {
    console.error(error);
  }
};

onMounted(loadSettings);
</script>

<style scoped src="./Settings.css"></style>
