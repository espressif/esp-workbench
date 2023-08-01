<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { join } from "@tauri-apps/api/path";
import PackagerOptions from "./components/PackagerOptions.vue";

const userHome = ref("");
const espIdfPath = ref("/Users/georgik/projects/esp-idf");
const espIdfVersion = ref("v5.1");
const espToolsPath = ref("/Users/georgik/.espressif");
const outputArchive = ref("/Users/georgik/esp-dev-env.zip");

async function updateProperties() {
  espToolsPath.value = userHome.value;
  espIdfPath.value = await join(espToolsPath.value, 'esp-idf', 'esp-idf-' + espIdfVersion.value);
  outputArchive.value = await join(espToolsPath.value, 'dist', 'esp-idf-' + espIdfVersion.value + '.zip');
}

function updateVersion(version: string) {
  espIdfVersion.value = version;
  updateProperties();
}

onMounted(() => {
  invoke("get_esp_idf_tools_dir").then((user_home) => {
    console.log("User home:" + user_home);
    userHome.value = user_home + "";
    updateProperties();
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
      v-model:output-archive="outputArchive"
      @update:esp-idf-version="(value: string) => updateVersion(value)"
    />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
