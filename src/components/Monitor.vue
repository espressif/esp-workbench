<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';

let logData = ref("");
let port = ref("");
type Payload = {
  pct: string,
}
onMounted(() => {
  port.value = decodeURIComponent(window.location.pathname.split("/")[2]); // assuming "/monitor/:port" route

  appWindow.listen('monitor-event', ({payload}) => logData.value += (payload as Payload).pct) + "\n";
  invoke('start_monitor', { port: port.value })
    .catch((error) => {
      console.error(error);
    });
});

onUnmounted(() => {
  invoke('stop_monitor')
    .catch((error) => {
      console.error(error);
    });
});

const stopMonitoring = () => {
  invoke('stop_monitor')
    .catch((error) => {
      console.error(error);
    });
};
</script>

<template>
  <div>
    <h2>Monitoring Port {{ port }}</h2>
    <button @click="stopMonitoring">Stop</button>
    <pre>{{ logData }}</pre>
  </div>
</template>
