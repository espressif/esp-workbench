<template>
  <div class="developer-portal-contribute">
    <h2>Developer Portal</h2>
    <div v-if="isDevPortalPresent">
      <AuthorList :repoPath="repoPath" />
      <ArticleList :repoPath="repoPath" />
      <button @click="launchHugo">Launch Hugo</button>
    </div>
    <div v-else>
      <p>Developer Portal directory is not present.</p>
      <CloneRepository
        :githubUsername="githubUsername"
        :initialRepoPath="repoPath"
        @cloningComplete="checkDevPortal"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import './DeveloperPortalContribute.css';
import AuthorList from './AuthorList.vue';
import ArticleList from './ArticleList.vue';
import CloneRepository from './CloneRepository.vue';

const isDevPortalPresent = ref(false);
const githubUsername = ref('');
const repoPath = ref('');

const checkDevPortal = async () => {
  try {
    isDevPortalPresent.value = await invoke('check_devportal', { repoPath: repoPath.value });
  } catch (error) {
    console.error(error);
  }
};

const loadSettings = async () => {
  try {
    const settings = await invoke('load_settings');
    githubUsername.value = settings.github_username || '';
    repoPath.value = settings.developer_portal_repo_path || '~/.espressif/developer-portal';
  } catch (error) {
    console.error(error);
  }
};

const launchHugo = async () => {
  try {
    await invoke('launch_hugo', { repo_path: repoPath.value });
  } catch (error) {
    console.error(error);
  }
};

onMounted(async () => {
  await loadSettings();
  await checkDevPortal();
});
</script>

<style scoped src="./DeveloperPortalContribute.css"></style>
