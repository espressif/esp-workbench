<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

let ports = ref([]);

// Extract the code for fetching ports into a separate method
const fetchPorts = () => {
  invoke('get_connected_serial_devices')
    .then((availablePorts) => {
      ports.value = availablePorts as any[];
    })
    .catch((error) => {
      console.error(error);
    });
}

onMounted(fetchPorts);
</script>

<template>
  <div>
    <h2>Connected ESP32 Boards</h2>
    <div v-if="ports.length > 0">
      <ul>
        <li v-for="(port, index) in ports" :key="index">
          {{ port.port_name }}
        </li>
      </ul>
    </div>
    <div v-else>
      No ESP32 boards connected...
    </div>
    <button class="refresh-button" @click="fetchPorts">Refresh</button>
  </div>
</template>

<style scoped>
.refresh-button {
  margin-top: 1rem;
  padding: 0.5rem 1rem;
  font-size: 1rem;
  background-color: #007bff;
  border: none;
  color: white;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  transition-duration: 0.4s;
  cursor: pointer;
}

.refresh-button:hover {
  background-color: #0056b3;
  color: white;
}
</style>
