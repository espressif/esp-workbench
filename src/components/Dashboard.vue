<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import EspIdfList from './EspIdfList.vue';

// import RustComponentStatus from './RustComponentStatus.vue';
// import SystemToolsStatus from './SystemToolsStatus.vue';
// import DiskSpaceStatus from './DiskSpaceStatus.vue';

let versions = ref([]);

function extractVersion(path: string): string {
  const parts = path.split('/');
  const lastPart = parts[parts.length - 1];
  const version = lastPart.replace('esp-idf-', '');
  return version;
}

function extractVersions(paths: string[]): string[] {
  return paths.map(extractVersion);
}

onMounted(() => {
  invoke("get_esp_idf_list").then((espIdfList) => {
    console.log("ESP-IDF List:" + espIdfList);
    versions.value = extractVersions((espIdfList as string[]));
  }).catch((error) => {
    console.error(error);
  });
});


</script>

<template>
  <div>
    <esp-idf-list :versions="versions" />
    <router-link  class="add-button" to="/esp-idf">+ Install new ESP-IDF version</router-link>
  </div>
</template>

<style scoped>
.add-button {
  display: inline-block;
  padding: 10px 20px;
  color: #fff;
  background-color: #007bff;
  border-radius: 5px;
  text-decoration: none;
  transition: background-color 0.2s ease;
}

.add-button:hover {
  background-color: #0056b3;
}
</style>