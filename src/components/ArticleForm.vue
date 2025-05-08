<template>
  <div class="modal" v-if="show">
    <div class="modal-content">
      <span class="close" @click="cancel">&times;</span>
      <h3>{{ formTitle }}</h3>
      <input v-model="articleData.title" placeholder="Title" />
      <textarea v-model="articleData.content" placeholder="Content"></textarea>
      <input v-model="articleData.date" placeholder="Date" />
      <label>
        <input type="checkbox" v-model="articleData.draft" />
        Draft
      </label>
      <button @click="save">Save</button>
      <button @click="cancel">Cancel</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from 'vue';
import './ArticleForm.css';

const props = defineProps({
  show: Boolean,
  article: Object,
  formTitle: String,
});

const emit = defineEmits(['save', 'cancel']);

const articleData = ref({
  title: '',
  content: '',
  date: '',
  draft: false,
  fileName: '',
});

const save = () => {
  emit('save', articleData.value);
};

const cancel = () => {
  emit('cancel');
};

watch(props, (newProps) => {
  articleData.value = { ...newProps.article };
});
</script>

<style scoped src="./ArticleForm.css"></style>
