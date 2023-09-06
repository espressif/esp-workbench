<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const ESP32_PIDS = ['0x1001', '0x55D4', '0x6001', '0x6010', '0x7523', '0xEA60'];
const ESP32_VIDS = ['0x0403', '0x10C4', '0x10C5', '0x1A86', '0x303A'];


let showAll = ref(false);

function formatPidVid(value: number): string {
  return "0x" + value.toString(16).toUpperCase().padStart(4, '0');
}

interface ConnectedPort {
  port_name: string;
  product: string;
  pid: number;
  vid: number;
}

let ports = ref<ConnectedPort[]>([]);

const fetchPorts = () => {
  invoke<ConnectedPort[]>('get_connected_serial_devices')
    .then((availablePorts) => {
      // availablePorts.forEach(port => {
      //   port.pid = formatPidVid(port.pid);
      //   port.vid = formatPidVid(port.vid);
      // });
      if (showAll.value) {
        ports.value = availablePorts;
      } else {
        ports.value = availablePorts.filter(port =>
          ESP32_PIDS.includes(formatPidVid(port.pid)) && ESP32_VIDS.includes(formatPidVid(port.vid))
        );
      }
    })
    .catch((error) => {
      console.error(error);
    });
};

onMounted(fetchPorts);
</script>

<template>
  <div>
    <h2>Connected ESP32 Boards</h2>
    <div v-if="ports.length > 0">
      <ul class="no-bullets">
        <li v-for="(port, index) in ports" :key="index"
            :class="{'gray-text': !ESP32_VIDS.includes(formatPidVid(port.vid))}">
          <span class="tooltip">
            {{ port.port_name }}
            <span class="tooltiptext">
              {{  port.product }}
              PID: {{ formatPidVid(port.pid) }} VID: {{ formatPidVid(port.vid) }}
            </span>
          </span>
          <div>
            <router-link :to="{ name: 'ESP Monitor', params: { portName: port.port_name }}">
              <button>Monitor</button>
            </router-link>
            <router-link :to="{ name: 'flash', params: { portName: port.port_name }}">
              <button>Flash</button>
            </router-link>
          </div>
        </li>
      </ul>
    </div>
    <div v-else>
      No ESP32 boards connected...
    </div>
    <div>
      <input type="checkbox" v-model="showAll" @change="fetchPorts"> Show all ports
    </div>
    <button class="refresh-button" @click="fetchPorts">Refresh</button>
  </div>
</template>

<style scoped>
.gray-text {
  color: gray;
}

.no-bullets {
  list-style-type: none;
}

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

ul {
  list-style-type: none; /* This will remove the dot bullet points */
}

/* Tooltip styling */
.tooltip {
  position: relative;
  display: inline-block;
  cursor: pointer;
}

.tooltip .tooltiptext {
  visibility: hidden;
  width: 120px;
  background-color: #555;
  color: #fff;
  text-align: center;
  border-radius: 6px;
  padding: 5px 0;
  position: absolute;
  z-index: 1;
  bottom: 125%; /* Position the tooltip above the text */
  left: 50%;
  margin-left: -60px; /* Use half of the width to center the tooltip */
  opacity: 0;
  transition: opacity 0.3s;
}

.tooltip:hover .tooltiptext {
  visibility: visible;
  opacity: 1;
}
</style>
