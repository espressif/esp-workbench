<script setup lang="ts">
import { ref, onMounted, nextTick, onBeforeUpdate, onUpdated } from 'vue';
// import { platform } from '@tauri-apps/api/os';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import {
        default as AnsiUp
    } from 'ansi_up';

let isWindows = ref(false);

let selectedToolchain = ref("xtensa");
let selectedVariant = ref("x86_64-pc-windows-msvc");
let installMsvc = ref(true);
let installMingw = ref(false);
let logs = ref("");
const logOutputRef = ref<HTMLDivElement | null>(null);
const ansi_up = new AnsiUp();

type ConsoleEvent = {
  message: string,
}

interface RustInstallOptions {
    selectedVariant?: string;
    installMsvc: boolean;
    installMingw: boolean;
}

const installRustSupport = () => {
  let rustInstallOptions = {
    selectedVariant: selectedVariant.value,
    installMsvc: selectedVariant.value === "x86_64-pc-windows-msvc" && installMsvc.value,
    installMingw: selectedVariant.value === "x86_64-pc-windows-gnu" && installMingw.value,
  } as RustInstallOptions;

  // Note: Tauri is using snake case for nested atributes, so it's necessary to make convertions
  invoke('install_rust_support', {installOptions: {
    selected_variant: rustInstallOptions.selectedVariant,
    install_msvc: rustInstallOptions.installMsvc,
    install_mingw: rustInstallOptions.installMingw,
  }})
    .then(() => {
      console.log("Rust Support Installed");
    })
    .catch((error) => {
      console.error(error);
    });
};

onBeforeUpdate(() => {
  // Capture current scroll position and height before the update.
  // If the current scroll position is already at the bottom, we'll scroll after the update.
  const shouldScroll = logOutputRef.value
    && logOutputRef.value.scrollTop + logOutputRef.value.clientHeight >= logOutputRef.value.scrollHeight;

  onUpdated(() => {
    // After the DOM is updated, scroll to the bottom if it was previously at the bottom
    if (shouldScroll && logOutputRef.value) {
      logOutputRef.value.scrollTop = logOutputRef.value.scrollHeight;
    }
  });
});

onMounted(async () => {
  appWindow.listen("rust-console", event => {
    const payload = event.payload as ConsoleEvent;
    console.log(payload.message);
    const htmlMessage = ansi_up.ansi_to_html(payload.message);
    logs.value += htmlMessage + "<br>";

    // Ensure the log scrolls to the bottom when a new message is received
    nextTick(() => {
      if (logOutputRef.value) {
        logOutputRef.value.scrollTop = logOutputRef.value.scrollHeight;
      }
    });
  });

  const platform = await invoke('get_platform');
  isWindows.value = platform === 'win32';
});



const updateSupportedChips = () => {
  // Depending on the selected toolchain, update the supported chips
  // This is just a dummy logic for demo purposes.
  supportedChips.value = (selectedToolchain.value === "xtensa")
    ? "ESP32, ESP32-S2, ESP32-S3"
    : "ESP32-C2, ESP32-C3, ESP32-C6, ESP32-H2";
}

const startInstallation = () => {
  installRustSupport();
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
        <option value="x86_64-pc-windows-msvc">MSVC (default)</option>
        <option value="x86_64-pc-windows-gnu">MinGW</option>
      </select>

      <!-- Checkbox for MSVC Dependencies -->
      <div v-if="selectedVariant === 'x86_64-pc-windows-msvc'">
        <input type="checkbox" v-model="installMsvc" id="installMsvcCheckbox">
        <label for="installMsvcCheckbox">Install VC Tools and Windows SDK</label>
      </div>

      <!-- Checkbox for MinGW Dependencies -->
      <div v-if="selectedVariant === 'x86_64-pc-windows-gnu'">
        <input type="checkbox" v-model="installMingw" id="installMingwCheckbox">
        <label for="installMingwCheckbox">Install MinGW Dependencies</label>
      </div>
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
      <div class="log-output" v-html="logs" ref="logOutputRef"></div>
    </div>

  </div>
</template>



<style scoped>
.rust-detail {
  display: flex;
  flex-direction: column;
  gap: 20px;
  /* max-width: 500px; */
  margin: auto;
}

.log-output {
  width: 100%;
  height: 300px;
  overflow-y: scroll;
  border: 1px solid #ccc;
  padding: 8px;
  white-space: pre-wrap;  /* Preserves whitespace & line breaks */
  font-family: monospace;
  background-color: #f8f8f8;
  text-align: left;
}
</style>
