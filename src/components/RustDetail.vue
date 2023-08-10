<script setup lang="ts">
import { ref, onMounted, nextTick, onBeforeUpdate, onUpdated } from 'vue';
// import { platform } from '@tauri-apps/api/os';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import LogConsole from './LogConsole.vue';

let isWindows = ref(false);

let selectedToolchain = ref("xtensa");
let selectedVariant = ref("x86_64-pc-windows-msvc");
let installMsvc = ref(true);
let installMingw = ref(false);

let isInstalling = ref(false);
let isAborted = ref(false);

interface RustInstallOptions {
    selectedVariant?: string;
    installMsvc: boolean;
    installMingw: boolean;
}

function abortBuild() {
  invoke("abort_build")
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
  isInstalling.value = false;
  isAborted.value = true;
}

const installRustSupport = () => {

  isInstalling.value = true;

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
    isInstalling.value = false;
    isAborted.value = false;
  })
  .catch((error) => {
    console.error(error);
    isInstalling.value = false;
    isAborted.value = false;
  });


};

onMounted(async () => {
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

    <div class="button-container">
      <!-- Installation Button -->
      <button v-if="!isInstalling" @click="installRustSupport()">Install</button>
      <button v-if="isInstalling" @click="abortBuild()">Cancel</button>
    </div>

    <LogConsole />

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
</style>
