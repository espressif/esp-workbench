<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { join } from "@tauri-apps/api/path";
import { getVersion } from '@tauri-apps/api/app';
import PackagerOptions from "./components/PackagerOptions.vue";

const appVersion = ref('');
const userHome = ref("");
const espIdfPath = ref("/Users/georgik/projects/esp-idf");
const espIdfVersion = ref("v5.1");
const espToolsPath = ref("/Users/georgik/.espressif");
const outputArchive = ref("/Users/georgik/esp-dev-env.zip");

async function updateProperties(path: string) {
  espToolsPath.value = path;
  espIdfPath.value = await join(espToolsPath.value, 'esp-idf', 'esp-idf-' + espIdfVersion.value);
  outputArchive.value = await join(espToolsPath.value, 'dist', 'esp-idf-' + espIdfVersion.value + '.zip');
}

function updateVersion(version: string) {
  espIdfVersion.value = version;
  updateProperties(espToolsPath.value);
}

async function fetchVersion() {
  appVersion.value = await getVersion();
}

onMounted(() => {
  invoke("get_esp_idf_tools_dir").then((user_home) => {
    fetchVersion();
    console.log("User home:" + user_home);
    userHome.value = user_home + "";
    updateProperties(userHome.value);
  }).catch((error) => {
    console.error(error);
  });
});

</script>

<template>
  <div class="container">
    <h1>ESP Helm</h1>
    <div>Navigate with ease in the world of ESP32</div>
    <PackagerOptions
      v-model:esp-idf-path="espIdfPath"
      v-model:esp-idf-version="espIdfVersion"
      v-model:esp-tools-path="espToolsPath"
      @update:esp-tools-path="(value: string) => updateProperties(value)"
      v-model:output-archive="outputArchive"
      @update:esp-idf-version="(value: string) => updateVersion(value)"
    />
    <div class="version">Version: {{ appVersion }}</div>
  </div>
</template>

<style scoped>
.version {
  position: fixed;
  bottom: 10px;
  right: 10px;
  font-size: xx-small;
}
</style>
