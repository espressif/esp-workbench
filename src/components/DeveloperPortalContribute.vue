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
          <button @click="editAuthor(author)">Edit</button>
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
        <div v-for="(social, index) in editAuthorData.social" :key="index">
          <input v-model="editAuthorData.social[index].url" placeholder="Social URL" />
        </div>
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
    const fileName = `${author.name.toLowerCase().replace(/ /g, '-')}.json`;
    await invoke('delete_author', { file_name: fileName });
    checkDevPortal(); // Refresh the list of authors
  } catch (error) {
    console.error(error);
  }
};

const editAuthor = (author: any) => {
  editAuthorData.value = { ...author };
  editFormTitle.value = `Edit Author: ${author.name}`;
  showEditForm.value = true;
};

const newAuthor = () => {
  editAuthorData.value = { name: '', bio: '', social: [] };
  editFormTitle.value = 'New Author';
  showEditForm.value = true;
};

const saveAuthor = async () => {
  try {
    const fileName = `${editAuthorData.value.name.toLowerCase().replace(/ /g, '-')}.json`;
    await invoke('save_author', { author: editAuthorData.value, file_name: fileName });
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
