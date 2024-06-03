<template>
  <div class="modal" v-if="show">
    <div class="modal-content">
      <span class="close" @click="cancel">&times;</span>
      <h3>{{ formTitle }}</h3>
      <input v-model="authorData.name" placeholder="Name" />
      <textarea v-model="authorData.bio" placeholder="Bio"></textarea>
      <div v-for="(social, index) in authorData.social" :key="index" class="social-input">
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
      <button @click="save">Save</button>
      <button @click="cancel">Cancel</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from 'vue';
import './AuthorForm.css';

const props = defineProps({
  show: Boolean,
  author: Object,
  formTitle: String,
});

const emit = defineEmits(['save', 'cancel']);

const authorData = ref({
  name: '',
  bio: '',
  social: [],
});

const addSocialField = () => {
  authorData.value.social.push({ key: '', url: '' });
};

const save = () => {
  emit('save', authorData.value);
};

const cancel = () => {
  emit('cancel');
};

watch(props, (newProps) => {
  authorData.value = { ...newProps.author };
});
</script>

<style scoped src="./AuthorForm.css"></style>
