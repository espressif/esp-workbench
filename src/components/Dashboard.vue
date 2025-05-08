<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import DiskUsage from './DiskUsage.vue';
import EspIdfList from './EspIdfList.vue';
import ConnectedDevicesList from "./ConnectedDevicesList.vue";
import RustDashboardTile from "./RustDashboardTile.vue";
import BooksTile from "./BooksTile.vue";
import DeveloperPortalTile from "./DeveloperPortalTile.vue";
import './Dashboard.css';

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
    </div>
    <div class="tile">
      <disk-usage />
    </div>
    <div class="tile">
      <connected-devices-list />
    </div>
    <div class="tile">
      <rust-dashboard-tile />
    </div>
    <div class="tile">
      <books-tile />
    </div>
    <div class="tile">
      <developer-portal-tile />
    </div>
  </div>
</template>
