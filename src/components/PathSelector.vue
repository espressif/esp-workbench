<script setup lang="ts">
import { ref } from "vue";
import { open } from '@tauri-apps/api/dialog';

const props = defineProps({
  path: String,
  title: String,
  allowDirectories: {
    type: Boolean,
    default: false,
  }
});

let emit = defineEmits(['update:path']);
const pathError = ref("");
const pathSelectorDialogOpen = ref(false);

async function openPathSelectorDialog() {
  pathSelectorDialogOpen.value = true;
  const selected = await open({
    directory: props.allowDirectories,
    multiple: false,
  });

  console.log(selected);
  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    emit('update:path', selected.toString());
  }

  pathSelectorDialogOpen.value = false;
}
</script>

<template>
  <div>
    <label for="path-input">{{ title }}: </label>
    <input
      class="path-input"
      :value="props.path"
      placeholder="Enter a path..."
      @input="value => emit('update:path', (value.target as HTMLSelectElement).value || '')"
    />
    <button type="button" @click="openPathSelectorDialog">...</button>
  </div>

  <p>{{ pathError }}</p>

  <tauri-dialog
    v-model="pathSelectorDialogOpen"
    title="Select a path"
    message="Select a path"
    filters='[{"name": "All Files", "extensions": ["*"]}]'
    onclose="pathSelectorDialogClosed"
    oncancel="pathSelectorDialogCancelled"
    onopen="pathSelectorDialogOpened"
  />
</template>
