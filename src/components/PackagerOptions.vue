<script setup lang="ts">
import { ref } from "vue";
import PathSelector from "./PathSelector.vue";

// const props = defineProps(['espToolsPath']);
const props = defineProps(['espToolsPath','espIdfPath','outputArchive']);

const espToolsPath = ref("/Users/georgik/.espressif");
const espIdfPath = ref("/Users/georgik/projects/esp-idf");
const outputArchive = ref("/Users/georgik/my-esp-idf.zip");

// Build package from ESP Tools and ESP-IDF
async function buildPackage() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  console.log(espToolsPath.value);
  console.log(espIdfPath.value);
  console.log(outputArchive.value);

  // await invoke("build_package", {
  //   esp_tools_path: espToolsPath.value,
  //   esp_idf_path: espIdfPath.value,
  //   output_archive: outputArchive.value,
  // });
}

</script>

<template>
  <form @submit.prevent="buildPackage">
    <PathSelector title="ESP Tools path" v-bind:path.sync="espToolsPath"/>
    <PathSelector title="ESP-IDF path" :path.sync="espIdfPath"/>
    <PathSelector title="Output archive" :path.sync="outputArchive"/>
    <button type="submit">Package</button>
  </form>
</template>