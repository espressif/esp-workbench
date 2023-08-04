<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import DiskUsage from './DiskUsage.vue';
import EspIdfList from './EspIdfList.vue';
import ConnectedDevicesList from "./ConnectedDevicesList.vue";

let versions = ref<string[]>([]);

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
  <div class="grid-container">
    <div class="tile">
      <esp-idf-list class="esp-idf-list" :versions="versions" />
      <router-link  class="add-button" to="/esp-idf">+ Install new ESP-IDF version</router-link>
    </div>
    <div class="tile">
      <disk-usage />
    </div>
    <div class="tile">
      <connected-devices-list />
    </div>
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

.esp-idf-list {
  padding-bottom: 1em;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 20px;
  padding: 20px;
}

.tile {
  border: 1px solid #ccc;
  padding: 20px;
  border-radius: 8px;
  background-color: #f8f8f8;
}
</style>