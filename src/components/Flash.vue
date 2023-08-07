<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import PathSelector from './PathSelector.vue';

let rawData = ref("");
let port = ref("");
let file = ref("");
let product = ref("");
let pid = ref("");
let vid = ref("");
let flashOffset = ref("0");

onMounted(() => {
  port.value = decodeURIComponent(window.location.pathname.split("/")[2]);
  // TODO: Fetch port info and fill product, pid, vid refs
});

const updateFilePath = (filePath: string) => {
  file.value = filePath;
};

const startFlashing = () => {
  if (file.value) {
    invoke('start_flash', { port: port.value, filePath: file.value, flashOffset: parseInt(flashOffset.value, 16) })
      .catch((error) => {
        console.error(error);
      });
  }
  // TODO: Start listening to 'flash-event'
};

const logData = computed(() => rawData.value.split('\n'));
</script>

<template>
  <div>
    <h2>Flashing Port {{ port }}</h2>
    <div>
      <label>Product: {{ product }}</label><br>
      <label>PID: {{ pid }}</label><br>
      <label>VID: {{ vid }}</label><br>
    </div>
    <div>
      <PathSelector
        title="Select BIN file:"
        :path="file"
        @update:path="updateFilePath"
      />
    </div>
    <div>
      <label for="flash-offset">Flash Offset (in Hex):</label>
      <input type="text" id="flash-offset" v-model="flashOffset">
    </div>
    <button @click="startFlashing">Flash</button>
    <pre class="console">
      <span v-for="(line, index) in logData" :key="index">{{ line }}<br /></span>
    </pre>
  </div>
</template>


<style scoped>
.console {
  background-color: black;
  color: limegreen;
  padding: 15px;
  max-height: 500px;
  overflow-y: scroll;
}
</style>
