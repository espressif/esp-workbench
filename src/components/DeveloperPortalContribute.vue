<template>
  <div>
    <h2>Developer Portal</h2>
    <div v-if="isDevPortalPresent">
      <button @click="launchHugo">Launch Hugo</button>
      <iframe v-show="isHugoRunning" src="http://localhost:1313" class="hugo-iframe"></iframe>
      <div v-if="authors.length">
        <h3>Authors</h3>
        <div v-for="(author, index) in authors" :key="author.name">
          <div v-if="editIndex === index">
            <input v-model="author.name" placeholder="Name" />
            <textarea v-model="author.bio" placeholder="Bio"></textarea>
            <div v-for="(social, socialIndex) in author.social" :key="socialIndex">
              <input v-model="social.url" placeholder="Social URL" />
            </div>
            <button @click="saveAuthor(index)">Save</button>
            <button @click="editIndex = -1">Cancel</button>
          </div>
          <div v-else>
            <h4>{{ author.name }}</h4>
            <p>{{ author.bio }}</p>
            <div v-for="social in author.social" :key="Object.keys(social)[0]">
              <a :href="Object.values(social)[0]">{{ Object.keys(social)[0] }}</a>
            </div>
            <button @click="editIndex = index">Edit</button>
          </div>
        </div>
      </div>
      <button @click="addNewAuthor">New Author</button>
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

const isDevPortalPresent = ref(false);
const authors = ref([]);
const editIndex = ref(-1);
const isHugoRunning = ref(false);

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
    await invoke('launch_hugo');
    isHugoRunning.value = true;
  } catch (error) {
    console.error(error);
  }
};

const saveAuthor = async (index: number) => {
  try {
    const author = authors.value[index];
    const fileName = `${author.name.toLowerCase().replace(/ /g, '-')}.json`;
    await invoke('save_author', { author, fileName });
    editIndex.value = -1;
    checkDevPortal();
    if (isHugoRunning.value) {
      await invoke('restart_hugo');
    }
  } catch (error) {
    console.error(error);
  }
};

const addNewAuthor = () => {
  authors.value.push({ name: '', bio: '', social: [{ linkedin: '', twitter: '' }] });
  editIndex.value = authors.value.length - 1;
};

onMounted(checkDevPortal);
</script>

<style scoped>
.hugo-iframe {
  width: 100%;
  height: 500px;
  border: none;
}
</style>
