<template>
  <div class="author-list">
    <button @click="newAuthor">New Author</button>
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
    <div v-else>
      <p>No authors available.</p>
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
import './AuthorList.css';
import AuthorForm from './AuthorForm.vue';

const props = defineProps({
  repoPath: String,
});

const authors = ref([]);
const showEditForm = ref(false);
const editAuthorData = ref({ name: '', bio: '', social: [] });
const editFormTitle = ref('');
let originalFileName = '';

const getFileName = (name: string) => {
  return `${name.toLowerCase().replace(/ /g, '-')}.json`;
};

const fetchAuthors = async () => {
  try {
    authors.value = await invoke('get_authors', { repoPath: props.repoPath });
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
    await invoke('delete_author', { fileName: fileName, repoPath: props.repoPath });
    fetchAuthors(); // Refresh the list of authors
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
    await invoke('save_author', { author: { ...authorData, social }, fileName: fileName, repoPath: props.repoPath });
    showEditForm.value = false;
    fetchAuthors(); // Refresh the list of authors
  } catch (error) {
    console.error(error);
  }
};

const cancelEdit = () => {
  showEditForm.value = false;
};

onMounted(fetchAuthors);
</script>

<style scoped src="./AuthorList.css"></style>
