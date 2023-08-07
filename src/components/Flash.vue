<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';

let rawData = ref("");
let port = ref("");
let file = ref(null);
let product = ref("");
let pid = ref("");
let vid = ref("");

onMounted(() => {
  port.value = decodeURIComponent(window.location.pathname.split("/")[2]); // assuming "/flash/:port" route
  // TODO: Fetch port info and fill product, pid, vid refs
});

const handleFileUpload = (event: { target: { files: null[]; }; }) => {
  file.value = event.target.files[0];
};

const startFlashing = () => {
  // TODO: Start flashing process and listen to 'flash-event'
};

const logData = computed(() => rawData.value.split('\n'));
</script>

<style scoped>
.console {
  background-color: black;
  color: limegreen;
  padding: 15px;
  max-height: 500px;
  overflow-y: scroll;
}
</style>

<template>
  <div>
    <h2>Flashing Port {{ port }}</h2>
    <div>
      <label>Product: {{ product }}</label><br>
      <label>PID: {{ pid }}</label><br>
      <label>VID: {{ vid }}</label><br>
    </div>
    <div>
      <label for="bin-file">Select BIN file:</label>
      <input type="file" id="bin-file" accept=".bin" v-on:change="handleFileUpload">
    </div>
    <button @click="startFlashing">Flash</button>
    <pre class="console">
      <span v-for="(line, index) in logData" :key="index">{{ line }}<br /></span>
    </pre>
  </div>
</template>
