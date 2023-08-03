<script setup lang="ts">
import { ref } from 'vue';
import EspIdfList from './EspIdfList.vue';
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
    <h1>ESP Helm</h1>
    <div>Navigate with ease in the world of ESP32</div>
    <esp-idf-list :versions="versions" />
    <!-- <rust-component-status />
    <system-tools-status />
    <disk-space-status /> -->
  </div>
</template>