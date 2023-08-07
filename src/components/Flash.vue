<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import PathSelector from './PathSelector.vue';

let rawData = ref("");
let port = ref("");
let file = ref("");
// let product = ref("");
// let pid = ref("");
// let vid = ref("");
let flashOffset = ref("0");
let progress = ref(0);
let total = ref(0);

type FlashProgressEvent = {
  count: number;
  total: number;
};

onMounted(() => {
  port.value = decodeURIComponent(window.location.pathname.split("/")[2]);
  appWindow.listen('flash-update', (event) => {
    const payload = event.payload as FlashProgressEvent;
    progress.value = (payload.count / payload.total) * 100;
    total.value = payload.total;
  });

  appWindow.listen('flash-finish', (event) => {

    progress.value = 100;
  });
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

// const logData = computed(() => rawData.value.split('\n'));
</script>

<template>
  <div>
    <h2>Flashing Port {{ port }}</h2>
    <!-- <div>
      <label>Product: {{ product }}</label><br>
      <label>PID: {{ pid }}</label><br>
      <label>VID: {{ vid }}</label><br>
    </div> -->
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

    <!-- Progress Bar -->
    <div class="progress">
      <div class="progress-bar" :style="{ width: progress + '%' }"></div>
    </div>

    <button @click="startFlashing">Flash</button>
    <router-link :to="{ name: 'ESP Monitor', params: { portName: port.value }}">
      <button>Monitor</button>
    </router-link>
    <!-- <pre class="console">
      <span v-for="(line, index) in logData" :key="index">{{ line }}<br /></span>
    </pre> -->
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


.progress {
  padding-top: 1em;
  padding-bottom: 1em;
  width: 100%;
  height: 20px;
  background-color: #f3f3f3;
  border-radius: 3px;
}

.progress-bar {
  height: 100%;
  background-color: #689f38;
  transition: width 0.2s;
}
</style>
