<script setup lang="ts">
// PathSelector component allows user to select a path from file system.
// It allows editing the path manually in the text field.
// It has also validation for the path and shows error message if the path is invalid.
// It also has a button to open a file selector dialog.

import { ref } from "vue";
import { open } from '@tauri-apps/api/dialog';
// import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps(['path','title'])

const pathRef = ref(props.path);
const pathError = ref("");
const pathSelectorDialogOpen = ref(false);

async function openPathSelectorDialog() {
  pathSelectorDialogOpen.value = true;
  const selected = await open({
    directory: true,
    multiple: false,
  });

  console.log(selected);
  if (Array.isArray(selected)) {
  // user selected multiple directories
  } else if (selected === null) {
  // user cancelled the selection
  } else {
      pathRef.value = selected.toString();
  }

  pathSelectorDialogOpen.value = false;
}

function validatePath() {
  if (pathRef === "") {
    pathError.value = "Path cannot be empty";
  } else {
    pathError.value = "";
  }
}

function pathChanged() {
  validatePath();
}

function pathSelectorDialogClosed() {
  validatePath();
}

function pathSelectorDialogCancelled() {
  pathRef.value = "";
  pathError.value = "";
}

function pathSelectorDialogOpened() {
  validatePath();
}
</script>

<template>
  <div class="row">
    <label for="path-input">{{ title }}</label>
    <input
      class="path-input"
      v-model="pathRef"
      placeholder="Enter a path..."
      @change="pathChanged"
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
