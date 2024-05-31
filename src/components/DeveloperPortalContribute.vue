<template>
  <div class="developer-portal-contribute">
    <h2>Developer Portal</h2>
    <div v-if="isDevPortalPresent">
      <button @click="newAuthor">New Author</button>
      <button @click="launchHugo">Launch Hugo</button>
      <div v-if="authors.length">
        <h3>Authors</h3>
        <div v-for="author in authors" :key="author.name" class="author">
          <h4>{{ author.name }}</h4>
          <p>{{ author.bio }}</p>
          <div v-for="social in author.social" :key="Object.keys(social)[0]">
            <a :href="Object.values(social)[0]">{{ Object.keys(social)[0] }}</a>
          </div>
          <button @click="editAuthor(author, getFileName(author.name))">Edit</button>
          <button @click="confirmDelete(author)">Delete</button>
        </div>
      </div>
    </div>
    <div v-else>
      <p>Developer Portal directory is not present.</p>
      <CloneRepository
        :githubUsername="githubUsername"
        :initialRepoPath="repoPath"
        @cloningComplete="checkDevPortal"
      />
    </div>

    <!-- Use the AuthorForm component -->
    <AuthorForm
      :show="showEditForm"
      :author="editAuthorData"
      :formTitle="editFormTitle"
      @save="saveAuthor"
      @cancel="cancelEdit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import './DeveloperPortalContribute.css';
import AuthorForm from './AuthorForm.vue';
import CloneRepository from './CloneRepository.vue';

const isDevPortalPresent = ref(false);
const authors = ref([]);
const showEditForm = ref(false);
const editAuthorData = ref({ name: '', bio: '', social: [] });
const editFormTitle = ref('');
let originalFileName = '';
let githubUsername = ref('');
let repoPath = ref('');

const getFileName = (name: string) => {
  return `${name.toLowerCase().replace(/ /g, '-')}.json`;
};

const checkDevPortal = async () => {
  try {
    isDevPortalPresent.value = await invoke('check_devportal', { repoPath: repoPath.value });
    if (isDevPortalPresent.value) {
      authors.value = await invoke('get_authors', { repoPath: repoPath.value });
    }
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

const confirmDelete = (author: any) => {
  if (confirm(`Are you sure you want to delete ${author.name}?`)) {
    deleteAuthor(author);
  }
};

const deleteAuthor = async (author: any) => {
  try {
    const fileName = getFileName(author.name);
    await invoke('delete_author', { fileName: fileName, repoPath: repoPath.value });
    checkDevPortal(); // Refresh the list of authors
  } catch (error) {
    console.error(error);
  }
};

const editAuthor = (author: any, fileName: string) => {
  editAuthorData.value = {
    name: author.name,
    bio: author.bio,
    social: author.social.map((s: any) => ({ key: Object.keys(s)[0], url: Object.values(s)[0] }))
  };
  originalFileName = fileName;
  editFormTitle.value = `Edit Author: ${author.name}`;
  showEditForm.value = true;
};

const newAuthor = () => {
  editAuthorData.value = { name: '', bio: '', social: [] };
  originalFileName = '';
  editFormTitle.value = 'New Author';
  showEditForm.value = true;
};

const saveAuthor = async (authorData: any) => {
  try {
    let fileName = originalFileName;
    if (!fileName) {
      fileName = getFileName(authorData.name);
    }
    const social = authorData.social.reduce((acc: any[], s: any) => {
      acc.push({ [s.key]: s.url });
      return acc;
    }, []);
    await invoke('save_author', { author: { ...authorData, social }, fileName: fileName, repoPath: repoPath.value });
    showEditForm.value = false;
    checkDevPortal(); // Refresh the list of authors
  } catch (error) {
    console.error(error);
  }
};

const cancelEdit = () => {
  showEditForm.value = false;
};

onMounted(async () => {
  await loadSettings();
  await checkDevPortal();
});
</script>

<style scoped src="./DeveloperPortalContribute.css"></style>
