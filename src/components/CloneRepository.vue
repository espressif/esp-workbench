<template>
  <div class="clone-repository">
    <h3>Clone Repository</h3>
    <div class="repo-option">
      <input
        type="text"
        :value="userRepoUrl"
        readonly
      />
      <button @click="cloneUserRepo">Clone User's Fork</button>
    </div>
    <div class="repo-option">
      <input
        type="text"
        value="git@github.com:espressif/developer-portal.git"
        readonly
      />
      <button @click="cloneMainRepo">Clone Main Repository</button>
    </div>
    <div class="repo-path">
      <label for="repo-path-input">Repository Path:</label>
      <input
        id="repo-path-input"
        v-model="repoPath"
        placeholder="Enter path to store the repository"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, defineEmits, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps({
  githubUsername: String,
  initialRepoPath: String,
});

const emit = defineEmits(['cloningComplete']);

const githubUsername = ref(props.githubUsername);
const repoPath = ref(props.initialRepoPath || '~/.espressif/developer-portal');
const userRepoUrl = ref(`git@github.com:${githubUsername.value}/developer-portal.git`);

watch(githubUsername, (newValue) => {
  userRepoUrl.value = `git@github.com:${newValue}/developer-portal.git`;
});

const loadSettings = async () => {
  try {
    const settings = await invoke('load_settings');
    githubUsername.value = settings.github_username || '';
    repoPath.value = settings.developer_portal_repo_path || '~/.espressif/developer-portal';
    userRepoUrl.value = `git@github.com:${githubUsername.value}/developer-portal.git`;
  } catch (error) {
    console.error(error);
  }
};

const cloneRepo = async (repoUrl) => {
  try {
    await invoke('save_settings', { githubUsername: githubUsername.value, developerPortalRepoPath: repoPath.value }); // Save the new path
    await invoke('clone_devportal_repo', { repoUrl: repoUrl, repoPath: repoPath.value });
    emit('cloningComplete');
  } catch (error) {
    console.error(error);
  }
};

const cloneUserRepo = () => {
  cloneRepo(userRepoUrl.value);
};

const cloneMainRepo = () => {
  cloneRepo('git@github.com:espressif/developer-portal.git');
};

onMounted(loadSettings);
</script>

<style scoped src="./CloneRepository.css"></style>
