<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { join } from "@tauri-apps/api/path";

import EspIdfList from './EspIdfList.vue';
import PackagerOptions from "./PackagerOptions.vue";

// import RustComponentStatus from './RustComponentStatus.vue';
// import SystemToolsStatus from './SystemToolsStatus.vue';
// import DiskSpaceStatus from './DiskSpaceStatus.vue';

let versions = ref([
  'v5.1',
  'v5.0.3',
  'v4.4.5',
  'v4.3.5',
  'v4.2.5',
]);
</script>

<template>
  <div>
    <esp-idf-list :versions="versions" />
    <router-link  class="add-button" to="/esp-idf">+ Add ESP-IDF</router-link>
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