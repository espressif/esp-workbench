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
      <button @click="cloneRepo">Clone Repository</button>
    </div>

    <!-- Modal for adding/editing authors -->
    <div v-if="showEditForm" class="modal">
      <div class="modal-content">
        <span class="close" @click="cancelEdit">&times;</span>
        <h3>{{ editFormTitle }}</h3>
        <input v-model="editAuthorData.name" placeholder="Name" />
        <textarea v-model="editAuthorData.bio" placeholder="Bio"></textarea>
        <div v-for="(social, index) in editAuthorData.social" :key="index" class="social-input">
          <select v-model="social.key">
            <option value="linkedin">LinkedIn</option>
            <option value="twitter">Twitter</option>
            <option value="instagram">Instagram</option>
            <option value="medium">Medium</option>
            <option value="github">GitHub</option>
            <option value="link">Link</option>
          </select>
          <input v-model="social.url" placeholder="Social URL" />
        </div>
        <button @click="addSocialField">Add Social Field</button>
        <button @click="saveAuthor">Save</button>
        <button @click="cancelEdit">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import './DeveloperPortalContribute.css';

const isDevPortalPresent = ref(false);
const authors = ref([]);
const showEditForm = ref(false);
const editAuthorData = ref({ name: '', bio: '', social: [] });
const editFormTitle = ref('');
let originalFileName = '';

const getFileName = (name: string) => {
  return `${name.toLowerCase().replace(/ /g, '-')}.json`;
};

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
    await invoke('clone_devportal_repo');
    checkDevPortal();
  } catch (error) {
    console.error(error);
  }
};

const launchHugo = async () => {
  try {
    await invoke('launch_hugo');
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
    await invoke('delete_author', { file_name: fileName });
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

const addSocialField = () => {
  editAuthorData.value.social.push({ key: '', url: '' });
};

const saveAuthor = async () => {
  try {
    let fileName = originalFileName;
    if (!fileName) {
      fileName = getFileName(editAuthorData.value.name);
    }
    const social = editAuthorData.value.social.reduce((acc: any[], s: any) => {
      acc.push({ [s.key]: s.url });
      return acc;
    }, []);
    await invoke('save_author', { author: { ...editAuthorData.value, social }, fileName: fileName });
    showEditForm.value = false;
    checkDevPortal(); // Refresh the list of authors
  } catch (error) {
    console.error(error);
  }
};

const cancelEdit = () => {
  showEditForm.value = false;
};

onMounted(checkDevPortal);
</script>

<style scoped src="./DeveloperPortalContribute.css"></style>
