<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { join } from "@tauri-apps/api/path";

import PackagerOptions from "./PackagerOptions.vue";

// import RustComponentStatus from './RustComponentStatus.vue';
// import SystemToolsStatus from './SystemToolsStatus.vue';
// import DiskSpaceStatus from './DiskSpaceStatus.vue';

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


onMounted(() => {
  invoke("get_esp_idf_tools_dir").then((user_home) => {
    console.log("User home:" + user_home);
    userHome.value = user_home + "";
    updateProperties(userHome.value);
  }).catch((error) => {
    console.error(error);
  });
});

</script>

<template>
  <div>
    <PackagerOptions
      v-model:esp-idf-path="espIdfPath"
      v-model:esp-idf-version="espIdfVersion"
      v-model:esp-tools-path="espToolsPath"
      @update:esp-tools-path="(value: string) => updateProperties(value)"
      v-model:output-archive="outputArchive"
      @update:esp-idf-version="(value: string) => updateVersion(value)"
    />
  </div>
</template>