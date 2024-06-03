<template>
  <div class="article-list">
    <button @click="newArticle">New Article</button>
    <div v-if="articles.length">
      <h3>Articles</h3>
      <div v-for="article in articles" :key="article.file_name" class="article">
        <h4>{{ article.title }}</h4>
        <p>{{ article.content.substring(0, 100) }}...</p>
        <button @click="editArticle(article)">Edit</button>
        <button @click="confirmDelete(article)">Delete</button>
      </div>
    </div>
    <div v-else>
      <p>No articles available.</p>
    </div>

    <!-- Use the ArticleForm component -->
    <ArticleForm
      :show="showEditForm"
      :article="editArticleData"
      :formTitle="editFormTitle"
      @save="saveArticle"
      @cancel="cancelEdit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import './ArticleList.css';
import ArticleForm from './ArticleForm.vue';

const props = defineProps({
  repoPath: String,
});

const articles = ref([]);
const showEditForm = ref(false);
const editArticleData = ref({ title: '', content: '', date: '', draft: false, file_name: '' });
const editFormTitle = ref('');

const fetchArticles = async () => {
  try {
    articles.value = await invoke('get_articles', { repoPath: props.repoPath });
  } catch (error) {
    console.error(error);
  }
};

const confirmDelete = (article: any) => {
  if (confirm(`Are you sure you want to delete ${article.title}?`)) {
    deleteArticle(article);
  }
};

const deleteArticle = async (article: any) => {
  try {
    await invoke('delete_article', { fileName: article.file_name, repoPath: props.repoPath });
    fetchArticles(); // Refresh the list of articles
  } catch (error) {
    console.error(error);
  }
};

const editArticle = (article: any) => {
  editArticleData.value = { ...article };
  editFormTitle.value = `Edit Article: ${article.title}`;
  showEditForm.value = true;
};

const newArticle = () => {
  editArticleData.value = { title: '', content: '', date: '', draft: false, file_name: '' };
  editFormTitle.value = 'New Article';
  showEditForm.value = true;
};

const saveArticle = async (articleData: any) => {
  try {
    await invoke('save_article', { article: articleData, repoPath: props.repoPath });
    showEditForm.value = false;
    fetchArticles(); // Refresh the list of articles
  } catch (error) {
    console.error(error);
  }
};

const cancelEdit = () => {
  showEditForm.value = false;
};

onMounted(fetchArticles);
</script>

<style scoped src="./ArticleList.css"></style>
