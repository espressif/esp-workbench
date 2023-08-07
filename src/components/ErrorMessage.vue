<template>
  <div v-if="message" class="error-message">
    <div class="dismiss-button" @click="dismiss">X</div>
    <p>{{ message }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({
  message: String,
});

let message = ref(props.message);
const dismiss = () => {
  message.value = '';
};
</script>

<style scoped>
.error-message {
  position: fixed;
  right: 0;
  top: 0;
  background: #f44336; /* Red */
  color: white;
  padding: 16px;
  border-radius: 10px;
  box-sizing: border-box;
  width: 300px;
}

.dismiss-button {
  position: absolute;
  right: 10px;
  top: 10px;
  cursor: pointer;
}
</style>
