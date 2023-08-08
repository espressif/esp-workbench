<script setup lang="ts">
import { ref, computed } from 'vue';

// For demo purposes, assuming detection logic exists
const isWindows = true;  // or false based on the OS detection logic

let selectedToolchain = ref("xtensa");
let selectedVariant = ref("msvc");
let logs = ref("");

const updateSupportedChips = () => {
  // Depending on the selected toolchain, update the supported chips
  // This is just a dummy logic for demo purposes.
  supportedChips.value = (selectedToolchain.value === "xtensa")
    ? "ESP32, ESP32-S2, ESP32-S3"
    : "ESP32-C2, ESP32-C3, ESP32-C6, ESP32-H2";
}

const startInstallation = () => {
  // Logic to invoke the installation through the `espup` crate
  // and update the logs.
}

let supportedChips = ref("ESP32, ESP32-S2, ESP-S3");  // Default for Xtensa
</script>

<template>
  <div class="rust-detail">

    <!-- Toolchain Selection -->
    <div>
      <label for="toolchain">Choose Toolchain:</label>
      <select v-model="selectedToolchain" @change="updateSupportedChips">
        <option value="xtensa">Xtensa</option>
        <option value="riscv">RISC-V</option>
      </select>
    </div>

    <!-- Variant Selection for Windows + Xtensa -->
    <div v-if="isWindows && selectedToolchain === 'xtensa'">
      <label for="variant">Choose Variant:</label>
      <select v-model="selectedVariant">
        <option value="msvc">MSVC (default)</option>
        <option value="mingw">MinGW</option>
      </select>
    </div>

    <!-- Display Supported Chips -->
    <div>
      <h3>Supported Chips:</h3>
      <p>{{ supportedChips }}</p>
    </div>

    <!-- Installation Button -->
    <button @click="startInstallation">Install</button>

    <!-- Display Installation Logs -->
    <div class="log-section">
      <h3>Installation Logs:</h3>
      <textarea readonly :value="logs"></textarea>
    </div>

  </div>
</template>


<style scoped>
.rust-detail {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-width: 500px;
  margin: auto;
}

.log-section textarea {
  width: 100%;
  height: 300px;
  resize: none;
}
</style>
